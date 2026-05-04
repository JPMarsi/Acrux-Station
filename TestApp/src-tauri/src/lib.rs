mod telemetry;
//mod modules;
pub mod modules;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(telemetry::create_initial_state())
        .invoke_handler(tauri::generate_handler![
            greet,
            telemetry::commands::get_telemetry,
            telemetry::commands::send_custom_command,
        
            telemetry::commands::list_serial_ports,
            telemetry::commands::get_serial_config,
            telemetry::commands::set_serial_config
        ])
        .setup(|app| {
            telemetry::setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
