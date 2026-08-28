pub mod container_telemetry;
pub mod parser;
pub mod pocketqube_telemetry;
pub mod protocol;
pub mod serial_runtime;
pub mod serial_service;
pub mod state;
pub mod ui_runtime;

use std::sync::Mutex;
use tauri::Manager;

use self::container_telemetry::ContainerTelemetry;
use self::pocketqube_telemetry::PocketQubeTelemetry;
use self::state::AppState;

use crate::modules::fileHandler;

pub fn create_default_container_telemetry() -> ContainerTelemetry {
    ContainerTelemetry {
        id: "1234C".into(),
        mission_time: "0.000".into(),
        packet_count: 0,
        command_count: 0,
        mode: "F".into(),
        altitude: 0.0,
        pressure: 0.0,
        temperature: 0.0,
        batt_v: 0.0,
        batt_i: 0.0,
        mech_state: "0x00".into(),
        state: "LAUNCH_PAD".into(),
        cmd_echo: "NONE".into(),
    }
}

pub fn create_default_pocketqube_telemetry() -> PocketQubeTelemetry {
    PocketQubeTelemetry {
        id: "1234P".into(),
        mode: "F".into(),
        mission_time: "0".into(),
        packet_count: 0,
        command_count: 0,
        altitude: 0.0,
        temperature: 0.0,
        pressure: 0.0,
        voltage: 0.0,
        current: 0.0,
        gnss_time: "00:00:00".into(),
        gnss_altitude: 0.0,
        gnss_latitude: 0.0,
        gnss_longitude: 0.0,
        gnss_sats: 0,
        rot_rate_x: 0.0,
        rot_rate_y: 0.0,
        rot_rate_z: 0.0,
        accel_x: 0.0,
        accel_y: 0.0,
        accel_z: 0.0,
        mag_x: 0.0,
        mag_y: 0.0,
        mag_z: 0.0,
        solar_1: 0.0,
        solar_2: 0.0,
        mech_state: "0x00".into(),
        cmd_echo: "NONE".into(),
        image_stabilization: "NONE".into(),
        science_exp: "NONE".into(),
    }
}

pub fn create_initial_state() -> AppState {
    AppState {
        container_telemetry: Mutex::new(create_default_container_telemetry()),
        pocketqube_telemetry: Mutex::new(create_default_pocketqube_telemetry()),
    }
}

pub fn setup(app: &tauri::App) {
    println!("Inicializando app sin abrir serial...");

    let csv_directory = app
        .path()
        .app_data_dir()
        .expect("No se pudo resolver la carpeta de datos de ACRUX")
        .join("csv");
    fileHandler::file_path(&csv_directory.to_string_lossy());
    fileHandler::file_csv_stop_recording();
}
