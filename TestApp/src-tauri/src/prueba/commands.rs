use tauri::State;

use super::state::AppState;
use super::telemetry::Telemetry;

#[tauri::command]
pub fn get_telemetry(state: State<AppState>) -> Telemetry {
    println!("get_telemetry() called");
    state.telemetry.lock().unwrap().clone()
}