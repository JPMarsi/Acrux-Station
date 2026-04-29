use tauri::State;

use super::state::AppState;
use super::telemetry::Telemetry;

#[tauri::command]
pub fn get_telemetry(state: State<AppState>) -> Telemetry {
    println!("get_telemetry() called");
    state.telemetry.lock().unwrap().clone()
}


#[tauri::command]
pub fn send_custom_command(command: String, state: State<AppState>) -> Result<String, String> {
    let clean_command = command.trim().to_uppercase();

    if clean_command.is_empty() {
        return Err("El comando está vacío".into());
    }

    let mut telemetry = state.telemetry.lock().unwrap();

    match clean_command.as_str() {
        "PING" => {
            telemetry.cmd_echo = "PING_OK".into();
            Ok("PONG".into())
        }
        "RESET" => {
            telemetry.state = "RESETTING".into();
            telemetry.cmd_echo = "RESET".into();
            Ok("Reset ejecutado".into())
        }
        "SAFE MODE" => {
            telemetry.mode = "SAFE".into();
            telemetry.cmd_echo = "SAFE MODE".into();
            Ok("Modo seguro activado".into())
        }
        "DEPLOY" => {
            telemetry.state = "DEPLOY".into();
            telemetry.cmd_echo = "DEPLOY".into();
            Ok("Secuencia de despliegue activada".into())
        }
        _ => Err(format!("Comando no reconocido: {}", clean_command))
    }
}