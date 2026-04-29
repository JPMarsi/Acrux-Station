pub mod commands;
pub mod emitter;
pub mod state;
pub mod telemetry;

use std::sync::Mutex;

use self::state::AppState;
use self::telemetry::Telemetry;

pub fn create_initial_state() -> AppState {
    AppState {
        telemetry: Mutex::new(Telemetry {
            team_id: "1999".into(),
            mission_time: "00:00:00".into(),
            packet_count: 0,
            mode: "FLIGHT".into(),
            state: "IDLE".into(),
            altitude: 0.0,
            temperature: 25.0,
            pressure: 1013.25,
            voltage: 12.0,
            current: 1.2,
            gyro_r: 0.0,
            gyro_p: 0.0,
            gyro_y: 0.0,
            accel_r: 0.0,
            accel_p: 0.0,
            accel_y: 0.0,
            gps_time: "00:00:00".into(),
            gps_altitude: 0.0,
            gps_latitude: -34.6037,
            gps_longitude: -58.3816,
            gps_sats: 5,
            cmd_echo: "NONE".into(),
            optional_data: "READY".into(),
        }),
    }
}

pub fn setup(app: &tauri::App) {
    println!("prueba::setup() called");
    let app_handle = app.handle().clone();
    emitter::start_mock_telemetry_emitter(&app_handle);
}