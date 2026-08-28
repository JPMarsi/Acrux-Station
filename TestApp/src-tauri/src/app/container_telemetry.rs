use serde::Serialize;

/// Paquete oficial 2027 del Container (13 campos).
#[derive(Debug, Clone, Serialize)]
pub struct ContainerTelemetry {
    pub id: String,
    pub mission_time: String,
    pub packet_count: u32,
    pub command_count: u32,
    pub mode: String,
    pub altitude: f32,
    pub pressure: f32,
    pub temperature: f32,
    pub batt_v: f32,
    pub batt_i: f32,
    pub mech_state: String,
    pub state: String,
    pub cmd_echo: String,
}
