use tauri::{AppHandle, Emitter, State};
use serialport::available_ports;


use super::serial_service;
use super::state::AppState;
use super::telemetry::Telemetry;
use crate::modules::fileHandler;
use crate::modules::serialController::{self, SerialConfig};

const TELEMETRY_FILE_ID: u32 = 1;
const TEAM_ID: u32 = 1234;

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

    if clean.to_uppercase().starts_with("CSV ") {
        return handle_csv_command(clean);
    }

    if clean.to_uppercase().starts_with("CMD,") {
        return serial_service::send_protocol_line(&app, clean);
    }

    Err(format!("Comando no reconocido: {}", clean))
}

fn handle_csv_command(command: &str) -> Result<String, String> {
    let normalized = command.to_ascii_uppercase();
    let parts: Vec<&str> = normalized.split_whitespace().collect();

    match parts.as_slice() {
        ["CSV", "START", format] => {
            let file = fileHandler::file_csv_start_recording(
                TELEMETRY_FILE_ID,
                TEAM_ID,
                format,
            )?;
            Ok(format!("Guardado CSV activado en formato {}: {}", format, file))
        }
        ["CSV", "STOP"] => {
            fileHandler::file_csv_stop_recording();
            Ok("Guardado CSV detenido".into())
        }
        _ => Err(
            "Comando CSV invalido. Usar: CSV START SESSION, CSV START MISSION o CSV STOP"
                .into(),
        ),
    }
}

#[tauri::command]
pub fn reset_app(state: State<AppState>, app: AppHandle) -> Result<Telemetry, String> {
    serial_service::reset_serial_session();

    let telemetry = super::create_default_telemetry();
    {
        let mut stored = state.telemetry.lock().map_err(|e| e.to_string())?;
        *stored = telemetry.clone();
    }

    fileHandler::file_path("./data");
    fileHandler::file_csv_stop_recording();

    let _ = app.emit("telemetry-update", telemetry.clone());

    Ok(telemetry)
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
