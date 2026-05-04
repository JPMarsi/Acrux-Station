use tauri::{AppHandle, State};
use serialport::available_ports;


use super::serial_service;
use super::state::AppState;
use super::telemetry::Telemetry;
use crate::modules::serialController::{self, SerialConfig};

#[tauri::command]
pub fn get_telemetry(state: State<AppState>) -> Telemetry {
    state.telemetry.lock().unwrap().clone()
}

#[tauri::command]
pub fn send_custom_command(
    command: String,
    _state: State<AppState>,
    app: AppHandle
) -> Result<String, String> {
    let clean = command.trim();

    if clean.is_empty() {
        return Err("El comando está vacío".into());
    }

    if clean.eq_ignore_ascii_case("START TELEMETRY")
        || clean.eq_ignore_ascii_case("CMD,1234,CX,ON")
    {
        return serial_service::start_telemetry(&app);
    }

    if clean.eq_ignore_ascii_case("END TELEMETRY")
        || clean.eq_ignore_ascii_case("CMD,1234,CX,OFF")
    {
        return serial_service::stop_telemetry();
    }

    if clean.to_uppercase().starts_with("CMD,") {
        return serial_service::send_protocol_line(&app, clean);
    }

    Err(format!("Comando no reconocido: {}", clean))
}


#[tauri::command]
pub fn list_serial_ports() -> Result<Vec<String>, String> {
    let ports = available_ports()
        .map_err(|e| format!("No se pudieron listar puertos: {}", e))?;

    Ok(ports.into_iter().map(|p| p.port_name).collect())
}

#[tauri::command]
pub fn get_serial_config() -> SerialConfig {
    serialController::get_serial_config()
}

#[tauri::command]
pub fn set_serial_config(port: String, baud_rate: u32) -> Result<SerialConfig, String> {
    serialController::set_serial_config(port, baud_rate)
}