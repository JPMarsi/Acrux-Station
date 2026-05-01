pub mod commands;
pub mod parser;
pub mod serial_runtime;
pub mod state;
pub mod telemetry;

use std::sync::Mutex;

use self::state::AppState;
use self::telemetry::Telemetry;

use crate::modules::fileHandler;
use crate::modules::serialController;

pub fn create_initial_state() -> AppState {
    AppState {
        telemetry: Mutex::new(Telemetry {
            team_id: "1234".into(),
            mission_time: "00:00:00".into(),
            packet_count: 0,
            mode: "F".into(),
            state: "BOOT".into(),
            altitude: 0.0,
            temperature: 0.0,
            pressure: 0.0,
            voltage: 0.0,
            current: 0.0,
            gyro_r: 0.0,
            gyro_p: 0.0,
            gyro_y: 0.0,
            accel_r: 0.0,
            accel_p: 0.0,
            accel_y: 0.0,
            gps_time: "00:00:00".into(),
            gps_altitude: 0.0,
            gps_latitude: 0.0,
            gps_longitude: 0.0,
            gps_sats: 0,
            cmd_echo: "NONE".into(),
            optional_data: "".into(),
        }),
    }
}

pub fn setup(app: &tauri::App) {
    println!("Inicializando serial real...");

    fileHandler::file_path("./data");
    fileHandler::file_csv_create_telemetry(1, 1234);

    serialController::serial_init("COM6", 115200)
        .expect("No se pudo abrir COM6 a 115200");

    let app_handle = app.handle().clone();
    serial_runtime::start_serial_telemetry_reader(&app_handle);

    serialController::serial_write_from_cmd_telemetry(1234, true);
}