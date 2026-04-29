use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Telemetry {
    pub team_id: String,
    pub mission_time: String,
    pub packet_count: u32,
    pub mode: String,
    pub state: String,
    pub altitude: f32,
    pub temperature: f32,
    pub pressure: f32,
    pub voltage: f32,
    pub current: f32,
    pub gyro_r: f32,
    pub gyro_p: f32,
    pub gyro_y: f32,
    pub accel_r: f32, 
    pub accel_p: f32,
    pub accel_y: f32,
    pub gps_time: String,
    pub gps_altitude: f32,
    pub gps_latitude: f64,
    pub gps_longitude: f64,
    pub gps_sats: u8,
    pub cmd_echo: String,
    pub optional_data: String,
}