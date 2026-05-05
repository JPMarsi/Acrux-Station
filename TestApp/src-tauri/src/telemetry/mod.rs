pub mod commands;
pub mod parser;
pub mod protocol;
pub mod serial_runtime;
pub mod serial_service;
pub mod state;
pub mod telemetry;

use std::sync::Mutex;

use self::state::AppState;
use self::telemetry::Telemetry;

use crate::modules::fileHandler;

pub fn create_default_telemetry() -> Telemetry {
    Telemetry {
        team_id: "1234".into(),
        mission_time: "00:00:00".into(),
        packet_count: 0,
        mode: "F".into(),
        state: "IDLE".into(),
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
    }
}

pub fn create_initial_state() -> AppState {
    AppState {
        telemetry: Mutex::new(create_default_telemetry()),
    }
}

pub fn setup(_app: &tauri::App) {
    println!("Inicializando app sin abrir serial...");

    fileHandler::file_path("./data");
    fileHandler::file_csv_create_telemetry(1, 1234);
}
