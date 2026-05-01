use std::{thread, time::Duration};

use tauri::{AppHandle, Emitter, Manager};

use super::parser::parse_telemetry_line;
use super::state::AppState;

use crate::modules::fileHandler;
use crate::modules::serialController;

pub fn start_serial_telemetry_reader(app: &AppHandle) {
    let app_handle = app.clone();

    thread::spawn(move || {
        loop {
            if let Some(line) = serialController::serial_read_line() {
                let clean = line.trim();

                if clean.is_empty() {
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }

                println!("SERIAL RX: {}", clean);

                if serialController::serial_detect(clean) == Some("telemetry") {
                    if let Some(telemetry) = parse_telemetry_line(clean) {
                        {
                            let state = app_handle.state::<AppState>();
                            let mut stored = state.telemetry.lock().unwrap();
                            *stored = telemetry.clone();
                        }

                        let _ = app_handle.emit("telemetry-update", telemetry);

                        fileHandler::file_csv_writeLine_telemetry(1, clean);
                    } else {
                        println!("No se pudo parsear telemetría: {}", clean);
                    }
                }
            }

            thread::sleep(Duration::from_millis(20));
        }
    });
}