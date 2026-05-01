mod prueba;
mod modules;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(prueba::create_initial_state())
        .invoke_handler(tauri::generate_handler![
            greet,
            prueba::commands::get_telemetry,
            prueba::commands::send_custom_command
        ])
        .setup(|app| {
            prueba::setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}