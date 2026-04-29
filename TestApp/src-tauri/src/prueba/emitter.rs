use std::{thread, time::Duration};

use tauri::{AppHandle, Emitter, Manager};

use super::state::AppState;
use super::telemetry::Telemetry;

pub fn start_mock_telemetry_emitter(app: &AppHandle) {
    let app_handle = app.clone();

    thread::spawn(move || {
        let mut counter: u32 = 0;

        loop {
            thread::sleep(Duration::from_secs(1));
            counter += 1;

            let telemetry = Telemetry {
                team_id: "1999".into(),
                mission_time: format!("00:00:{counter:02}"),
                packet_count: counter,
                mode: "FLIGHT".into(),
                state: if counter % 2 == 0 { "ASCENT".into() } else { "IDLE".into() },
                altitude: 1000.0 + counter as f32 * 2.5,
                temperature: 25.0 + counter as f32 * 0.1,
                pressure: 1013.25 - counter as f32 * 0.2,
                voltage: 12.0 - counter as f32 * 0.01,
                current: 1.2 + counter as f32 * 0.01,
                gyro_r: counter as f32 * 0.3,
                gyro_p: counter as f32 * 0.2,
                gyro_y: counter as f32 * 0.1,
                accel_r: counter as f32 * 0.05,
                accel_p: counter as f32 * 0.04,
                accel_y: counter as f32 * 0.03,
                gps_time: format!("00:00:{counter:02}"),
                gps_altitude: 950.0 + counter as f32 * 1.7,
                gps_latitude: -34.6037 + counter as f64 * 0.0001,
                gps_longitude: -58.3816 + counter as f64 * 0.0001,
                gps_sats: 5,
                cmd_echo: format!("CMD_{counter}"),
                optional_data: format!("TEST_{counter}"),
            };

            {
                let state = app_handle.state::<AppState>();
                let mut stored = state.telemetry.lock().unwrap();
                *stored = telemetry.clone();
            }

            

            let _ = app_handle.emit("telemetry-update", telemetry);
        }
    });
}