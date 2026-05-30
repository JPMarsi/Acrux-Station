mod app;
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
        .manage(app::create_initial_state())
        .invoke_handler(tauri::generate_handler![
            //greet,
            app::ui_runtime::get_telemetry,
            app::ui_runtime::reset_app,
            app::ui_runtime::send_custom_command,

            app::ui_runtime::list_serial_ports,
            app::ui_runtime::get_serial_config,
            app::ui_runtime::set_serial_config
        ])
        .setup(|app| {
            app::setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
