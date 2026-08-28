use serde::Serialize;
use serialport::available_ports;
use tauri::{AppHandle, Emitter, State};

use super::container_telemetry::ContainerTelemetry;
use super::pocketqube_telemetry::PocketQubeTelemetry;
use super::serial_service;
use super::state::AppState;
use crate::modules::fileHandler;
use crate::modules::serialController::{self, SerialConfig};

const TEAM_ID: u32 = 1234;

#[derive(Clone, Serialize)]
pub struct TelemetrySnapshot {
    pub container: ContainerTelemetry,
    pub pocketqube: PocketQubeTelemetry,
}

#[derive(Clone, Serialize)]
pub struct CsvRecordingStatus {
    pub active: bool,
}

fn current_csv_status() -> CsvRecordingStatus {
    CsvRecordingStatus {
        active: fileHandler::file_csv_is_recording(),
    }
}

#[tauri::command]
pub fn get_csv_recording_status() -> CsvRecordingStatus {
    current_csv_status()
}

#[tauri::command]
pub fn get_telemetry(state: State<AppState>) -> TelemetrySnapshot {
    TelemetrySnapshot {
        container: state.container_telemetry.lock().unwrap().clone(),
        pocketqube: state.pocketqube_telemetry.lock().unwrap().clone(),
    }
}

#[tauri::command]
pub fn send_custom_command(
    command: String,
    _state: State<AppState>,
    app: AppHandle,
) -> Result<String, String> {
    let clean = command.trim();

    if clean.is_empty() {
        return Err("El comando está vacío".into());
    }

    if clean.eq_ignore_ascii_case("START TELEMETRY") || clean.eq_ignore_ascii_case("CMD,1234,CX,ON")
    {
        return serial_service::start_telemetry(&app);
    }

    if clean.eq_ignore_ascii_case("END TELEMETRY") || clean.eq_ignore_ascii_case("CMD,1234,CX,OFF")
    {
        return serial_service::stop_telemetry();
    }

    if clean.to_uppercase().starts_with("CSV ") {
        return handle_csv_command(clean, &app);
    }

    if clean.to_uppercase().starts_with("CMD,") {
        return serial_service::send_protocol_line(&app, clean);
    }

    Err(format!("Comando no reconocido: {}", clean))
}

fn handle_csv_command(command: &str, app: &AppHandle) -> Result<String, String> {
    let normalized = command.to_ascii_uppercase();
    let parts: Vec<&str> = normalized.split_whitespace().collect();

    match parts.as_slice() {
        ["CSV", "START"] | ["CSV", "START", "MISSION"] | ["CSV", "START", "FLIGHT"] => {
            let (container, pocketqube) =
                fileHandler::file_csv_start_official_flight_recording(TEAM_ID)?;
            let _ = app.emit("csv-recording-update", current_csv_status());
            Ok(format!(
                "Grabación oficial iniciada: Container={} | PocketQube={}",
                container, pocketqube
            ))
        }
        ["CSV", "STOP"] => {
            fileHandler::file_csv_stop_recording();
            let _ = app.emit("csv-recording-update", current_csv_status());
            Ok("Guardado CSV detenido".into())
        }
        _ => Err("Comando CSV inválido. Usar: CSV START o CSV STOP".into()),
    }
}

#[tauri::command]
pub fn reset_app(state: State<AppState>, app: AppHandle) -> Result<TelemetrySnapshot, String> {
    serial_service::reset_serial_session();

    let container = super::create_default_container_telemetry();
    let pocketqube = super::create_default_pocketqube_telemetry();
    {
        *state
            .container_telemetry
            .lock()
            .map_err(|e| e.to_string())? = container.clone();
        *state
            .pocketqube_telemetry
            .lock()
            .map_err(|e| e.to_string())? = pocketqube.clone();
    }

    fileHandler::file_csv_stop_recording();
    let _ = app.emit("csv-recording-update", current_csv_status());

    let _ = app.emit("container-telemetry-update", container.clone());
    let _ = app.emit("pocketqube-telemetry-update", pocketqube.clone());

    Ok(TelemetrySnapshot {
        container,
        pocketqube,
    })
}

#[tauri::command]
pub fn list_serial_ports() -> Result<Vec<String>, String> {
    let ports = available_ports().map_err(|e| format!("No se pudieron listar puertos: {}", e))?;

    let filtered = ports
        .into_iter()
        .map(|p| p.port_name)
        .filter(|name| {
            name.starts_with("COM") || name.contains("ttyUSB") || name.contains("ttyACM")
        })
        .collect();

    Ok(filtered)
}

#[tauri::command]
pub fn get_serial_config() -> SerialConfig {
    serialController::get_serial_config()
}

#[tauri::command]
pub fn set_serial_config(
    espnow_port: String,
    espnow_baud_rate: u32,
    xbee_port: String,
    xbee_baud_rate: u32,
) -> Result<SerialConfig, String> {
    serialController::set_serial_config(SerialConfig {
        espnow_port,
        espnow_baud_rate,
        xbee_port,
        xbee_baud_rate,
    })
}
