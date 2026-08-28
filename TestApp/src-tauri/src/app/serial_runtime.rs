use std::collections::{HashSet, VecDeque};
use std::thread;
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

use super::parser::{
    identify_telemetry_source, parse_container_telemetry, parse_pocketqube_telemetry,
    TelemetrySource,
};
use super::serial_service;
use super::state::AppState;
use crate::modules::fileHandler;
use crate::modules::serialController::{self, LinkKind};

const PRIMARY_TIMEOUT: Duration = Duration::from_millis(2_500);
const PRIMARY_RECOVERY_PACKETS: u8 = 4;
const RECONNECT_INTERVAL: Duration = Duration::from_secs(2);
const STATUS_INTERVAL: Duration = Duration::from_millis(500);
const DEDUP_CAPACITY: usize = 512;

#[derive(Default)]
struct PayloadLinkState {
    active: Option<LinkKind>,
    last_espnow: Option<Instant>,
    espnow_recovery_streak: u8,
}

impl PayloadLinkState {
    fn accepts(&mut self, link: LinkKind, now: Instant) -> bool {
        match link {
            LinkKind::EspNow => {
                self.last_espnow = Some(now);
                if self.active == Some(LinkKind::XBee) {
                    self.espnow_recovery_streak = self.espnow_recovery_streak.saturating_add(1);
                    if self.espnow_recovery_streak >= PRIMARY_RECOVERY_PACKETS {
                        self.active = Some(LinkKind::EspNow);
                        self.espnow_recovery_streak = 0;
                    }
                } else {
                    self.active = Some(LinkKind::EspNow);
                    self.espnow_recovery_streak = 0;
                }
                self.active == Some(LinkKind::EspNow)
            }
            LinkKind::XBee => {
                let primary_stale = self
                    .last_espnow
                    .map(|last| now.duration_since(last) > PRIMARY_TIMEOUT)
                    .unwrap_or(true);
                if self.active != Some(LinkKind::EspNow) || primary_stale {
                    self.active = Some(LinkKind::XBee);
                    self.espnow_recovery_streak = 0;
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Default)]
struct Deduplicator {
    order: VecDeque<String>,
    keys: HashSet<String>,
}

impl Deduplicator {
    fn insert_new(&mut self, key: String) -> bool {
        if !self.keys.insert(key.clone()) {
            return false;
        }
        self.order.push_back(key);
        if self.order.len() > DEDUP_CAPACITY {
            if let Some(oldest) = self.order.pop_front() {
                self.keys.remove(&oldest);
            }
        }
        true
    }
}

#[derive(Clone, Serialize)]
pub struct LinkStatus {
    pub espnow_connected: bool,
    pub xbee_connected: bool,
    pub container_source: String,
    pub pocketqube_source: String,
    pub espnow_last_packet_ms: Option<u64>,
    pub xbee_last_packet_ms: Option<u64>,
}

fn source_label(source: Option<LinkKind>) -> String {
    source
        .map(LinkKind::label)
        .unwrap_or("SIN DATOS")
        .to_string()
}

fn packet_key(line: &str, source: TelemetrySource) -> Option<String> {
    let fields: Vec<&str> = telemetry_csv_line(line).split(',').collect();
    let packet_index = match source {
        TelemetrySource::Container => 2,
        TelemetrySource::PocketQube => 3,
    };
    Some(format!("{}:{}", fields.first()?, fields.get(packet_index)?))
}

fn telemetry_csv_line(line: &str) -> &str {
    line.split(",,").next().unwrap_or(line).trim()
}

fn process_line(
    app: &AppHandle,
    link: LinkKind,
    line: &str,
    container_link: &mut PayloadLinkState,
    pocketqube_link: &mut PayloadLinkState,
    deduplicator: &mut Deduplicator,
) -> bool {
    let clean = line.trim();
    if clean.is_empty() || !clean.contains(",,telemetry") {
        return false;
    }
    let Some(source) = identify_telemetry_source(clean) else {
        return false;
    };
    let now = Instant::now();
    let accepted_link = match source {
        TelemetrySource::Container => container_link.accepts(link, now),
        TelemetrySource::PocketQube => pocketqube_link.accepts(link, now),
    };
    if !accepted_link {
        return true;
    }
    let Some(key) = packet_key(clean, source) else {
        return false;
    };
    if !deduplicator.insert_new(key) {
        return true;
    }
    match source {
        TelemetrySource::Container => {
            let Some(telemetry) = parse_container_telemetry(clean) else {
                return false;
            };
            *app.state::<AppState>().container_telemetry.lock().unwrap() = telemetry.clone();
            let _ = app.emit("container-telemetry-update", telemetry);
            fileHandler::file_csv_writeLine_telemetry_if_recording(
                fileHandler::CONTAINER_FILE_ID,
                telemetry_csv_line(clean),
            );
        }
        TelemetrySource::PocketQube => {
            let Some(telemetry) = parse_pocketqube_telemetry(clean) else {
                return false;
            };
            *app.state::<AppState>().pocketqube_telemetry.lock().unwrap() = telemetry.clone();
            let _ = app.emit("pocketqube-telemetry-update", telemetry);
            fileHandler::file_csv_writeLine_telemetry_if_recording(
                fileHandler::POCKETQUBE_FILE_ID,
                telemetry_csv_line(clean),
            );
        }
    }
    true
}

pub fn start_serial_telemetry_reader(app: &AppHandle) {
    let app_handle = app.clone();
    thread::spawn(move || {
        let mut container_link = PayloadLinkState::default();
        let mut pocketqube_link = PayloadLinkState::default();
        let mut deduplicator = Deduplicator::default();
        let mut last_packet = [None, None];
        let mut last_reconnect = Instant::now();
        let mut last_status = Instant::now();

        loop {
            let now = Instant::now();
            if serial_service::is_telemetry_active()
                && now.duration_since(last_reconnect) >= RECONNECT_INTERVAL
            {
                for link in [LinkKind::EspNow, LinkKind::XBee] {
                    if !serialController::serial_is_open(link) {
                        let _ = serialController::serial_init_link(link);
                    }
                }
                last_reconnect = now;
            }

            for (index, link) in [LinkKind::EspNow, LinkKind::XBee].into_iter().enumerate() {
                match serialController::serial_read_lines(link) {
                    Ok(lines) => {
                        for line in lines {
                            if process_line(
                                &app_handle,
                                link,
                                &line,
                                &mut container_link,
                                &mut pocketqube_link,
                                &mut deduplicator,
                            ) {
                                last_packet[index] = Some(Instant::now());
                            }
                        }
                    }
                    Err(error) => eprintln!("{}", error),
                }
            }

            if now.duration_since(last_status) >= STATUS_INTERVAL {
                let command_link = if container_link.active == Some(LinkKind::XBee)
                    && pocketqube_link.active == Some(LinkKind::XBee)
                {
                    LinkKind::XBee
                } else {
                    LinkKind::EspNow
                };
                serialController::set_preferred_command_link(command_link);
                let age = |last: Option<Instant>| {
                    last.map(|instant| now.duration_since(instant).as_millis() as u64)
                };
                let _ = app_handle.emit(
                    "link-status-update",
                    LinkStatus {
                        espnow_connected: serialController::serial_is_open(LinkKind::EspNow),
                        xbee_connected: serialController::serial_is_open(LinkKind::XBee),
                        container_source: source_label(container_link.active),
                        pocketqube_source: source_label(pocketqube_link.active),
                        espnow_last_packet_ms: age(last_packet[0]),
                        xbee_last_packet_ms: age(last_packet[1]),
                    },
                );
                last_status = now;
            }
            thread::sleep(Duration::from_millis(15));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{packet_key, Deduplicator, PayloadLinkState, PRIMARY_TIMEOUT};
    use crate::app::parser::TelemetrySource;
    use crate::modules::serialController::LinkKind;
    use std::time::Instant;

    #[test]
    fn packet_keys_follow_each_official_format() {
        assert_eq!(
            packet_key("1234C,1.0,27,F", TelemetrySource::Container).unwrap(),
            "1234C:27"
        );
        assert_eq!(
            packet_key("1234P,F,1.0,31", TelemetrySource::PocketQube).unwrap(),
            "1234P:31"
        );
    }

    #[test]
    fn duplicate_packet_is_rejected() {
        let mut dedup = Deduplicator::default();
        assert!(dedup.insert_new("1234C:1".into()));
        assert!(!dedup.insert_new("1234C:1".into()));
    }

    #[test]
    fn xbee_takes_over_after_primary_timeout() {
        let mut state = PayloadLinkState::default();
        let start = Instant::now();
        assert!(state.accepts(LinkKind::EspNow, start));
        assert!(!state.accepts(LinkKind::XBee, start));
        assert!(state.accepts(
            LinkKind::XBee,
            start + PRIMARY_TIMEOUT + std::time::Duration::from_millis(1)
        ));
    }

    #[test]
    fn espnow_requires_four_packets_to_recover() {
        let mut state = PayloadLinkState::default();
        let start = Instant::now();
        assert!(state.accepts(LinkKind::XBee, start));
        for offset in 1..4 {
            assert!(!state.accepts(
                LinkKind::EspNow,
                start + std::time::Duration::from_millis(offset)
            ));
        }
        assert!(state.accepts(
            LinkKind::EspNow,
            start + std::time::Duration::from_millis(4)
        ));
    }
}
