use tauri::State;

use super::state::AppState;
use super::telemetry::Telemetry;
use crate::modules::serialController;

#[tauri::command]
pub fn get_telemetry(state: State<AppState>) -> Telemetry {
    state.telemetry.lock().unwrap().clone()
}

#[tauri::command]
pub fn send_custom_command(command: String, _state: State<AppState>) -> Result<String, String> {
    let clean = command.trim().to_uppercase();

    if clean.is_empty() {
        return Err("El comando está vacío".into());
    }

    match clean.as_str() {
        "START TELEMETRY" => {
            serialController::serial_write_from_cmd_telemetry(1234, true);
            Ok("CMD,1234,CX,ON enviado".into())
        }
        "END TELEMETRY" => {
            serialController::serial_write_from_cmd_telemetry(1234, false);
            Ok("CMD,1234,CX,OFF enviado".into())
        }
        "CAL ALTITUDE" => {
            serialController::serial_write_from_cmd_altitude_set_zero(1234);
            Ok("CMD,1234,CAL enviado".into())
        }
        _ => Err(format!("Comando no mapeado todavía: {}", clean)),
    }
}