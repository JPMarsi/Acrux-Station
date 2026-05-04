use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

use tauri::AppHandle;

use crate::modules::serialController;

use super::protocol;
use super::serial_runtime;

static SERIAL_RUNTIME_STARTED: OnceLock<()> = OnceLock::new();
static SERIAL_OPENED: AtomicBool = AtomicBool::new(false);
static TELEMETRY_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn ensure_serial_ready(app: &AppHandle) -> Result<(), String> {
    if !SERIAL_OPENED.load(Ordering::SeqCst) {
        let config = serialController::get_serial_config();

        serialController::serial_init(&config.selected_port, config.baud_rate)
            .map_err(|e| format!(
                "No se pudo abrir {} a {}: {}",
                config.selected_port, config.baud_rate, e
            ))?;

        SERIAL_OPENED.store(true, Ordering::SeqCst);
    }

    if SERIAL_RUNTIME_STARTED.get().is_none() {
        serial_runtime::start_serial_telemetry_reader(app);
        let _ = SERIAL_RUNTIME_STARTED.set(());
    }

    Ok(())
}

pub fn start_telemetry(app: &AppHandle) -> Result<String, String> {
    ensure_serial_ready(app)?;

    let line = protocol::build_start_telemetry();
    serialController::serial_write_raw(&line)
        .map_err(|e| format!("No se pudo enviar comando serial: {}", e))?;

    TELEMETRY_ACTIVE.store(true, Ordering::SeqCst);

    Ok(format!("Enviado: {}", line))
}

pub fn stop_telemetry() -> Result<String, String> {
    if !SERIAL_OPENED.load(Ordering::SeqCst) {
        TELEMETRY_ACTIVE.store(false, Ordering::SeqCst);
        return Ok("La telemetría ya estaba detenida".into());
    }

    let line = protocol::build_stop_telemetry();
    serialController::serial_write_raw(&line)
        .map_err(|e| format!("No se pudo enviar comando serial: {}", e))?;

    TELEMETRY_ACTIVE.store(false, Ordering::SeqCst);

    Ok(format!("Enviado: {}", line))
}

pub fn send_protocol_line(app: &AppHandle, line: &str) -> Result<String, String> {
    ensure_serial_ready(app)?;

    serialController::serial_write_raw(line)
        .map_err(|e| format!("No se pudo enviar comando serial: {}", e))?;

    Ok(format!("Enviado: {}", line))
}

pub fn is_telemetry_active() -> bool {
    TELEMETRY_ACTIVE.load(Ordering::SeqCst)
}