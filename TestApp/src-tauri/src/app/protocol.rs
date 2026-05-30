pub const TEAM_ID: u16 = 1234;

pub fn build_start_telemetry() -> String {
    format!("CMD,{},CX,ON", TEAM_ID)
}

pub fn build_stop_telemetry() -> String {
    format!("CMD,{},CX,OFF", TEAM_ID)
}