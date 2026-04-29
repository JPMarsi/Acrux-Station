use std::sync::Mutex;

use super::telemetry::Telemetry;

pub struct AppState {
    pub telemetry: Mutex<Telemetry>,
}