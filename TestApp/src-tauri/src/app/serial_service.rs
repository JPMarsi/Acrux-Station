use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

use tauri::AppHandle;

use crate::modules::serialController::{self, LinkKind};

use super::protocol;
use super::serial_runtime;

static SERIAL_RUNTIME_STARTED: OnceLock<()> = OnceLock::new();
static TELEMETRY_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn ensure_serial_ready(app: &AppHandle) -> Result<String, String> {
    let mut opened = Vec::new();
    let mut errors = Vec::new();
    for link in [LinkKind::EspNow, LinkKind::XBee] {
        match serialController::serial_init_link(link) {
            Ok(()) => opened.push(link.label()),
            Err(error) => errors.push(error),
        }
    }
    if opened.is_empty() {
        return Err(errors.join(" | "));
    }
    if SERIAL_RUNTIME_STARTED.get().is_none() {
        serial_runtime::start_serial_telemetry_reader(app);
        let _ = SERIAL_RUNTIME_STARTED.set(());
    }
    Ok(if errors.is_empty() {
        format!("Enlaces abiertos: {}", opened.join(" + "))
    } else {
        format!(
            "Enlaces abiertos: {} | {}",
            opened.join(" + "),
            errors.join(" | ")
        )
    })
}

pub fn start_telemetry(app: &AppHandle) -> Result<String, String> {
    let connection_status = ensure_serial_ready(app)?;
    let line = protocol::build_start_telemetry();
    let link = serialController::serial_write_preferred(&line)
        .map_err(|e| format!("No se pudo enviar comando serial: {}", e))?;
    TELEMETRY_ACTIVE.store(true, Ordering::SeqCst);
    Ok(format!(
        "{} | Enviado por {}: {}",
        connection_status,
        link.label(),
        line
    ))
}

pub fn stop_telemetry() -> Result<String, String> {
    if !serialController::serial_is_open(LinkKind::EspNow)
        && !serialController::serial_is_open(LinkKind::XBee)
    {
        TELEMETRY_ACTIVE.store(false, Ordering::SeqCst);
        return Ok("La telemetría ya estaba detenida".into());
    }
    let line = protocol::build_stop_telemetry();
    let link = serialController::serial_write_preferred(&line)
        .map_err(|e| format!("No se pudo enviar comando serial: {}", e))?;
    TELEMETRY_ACTIVE.store(false, Ordering::SeqCst);
    Ok(format!("Enviado por {}: {}", link.label(), line))
}

pub fn reset_serial_session() {
    TELEMETRY_ACTIVE.store(false, Ordering::SeqCst);
    serialController::serial_close_all();
}

pub fn send_protocol_line(app: &AppHandle, line: &str) -> Result<String, String> {
    ensure_serial_ready(app)?;
    let link = serialController::serial_write_preferred(line)
        .map_err(|e| format!("No se pudo enviar comando serial: {}", e))?;
    Ok(format!("Enviado por {}: {}", link.label(), line))
}

pub fn is_telemetry_active() -> bool {
    TELEMETRY_ACTIVE.load(Ordering::SeqCst)
}
