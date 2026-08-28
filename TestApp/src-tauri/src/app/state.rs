use std::sync::Mutex;

use super::container_telemetry::ContainerTelemetry;
use super::pocketqube_telemetry::PocketQubeTelemetry;

pub struct AppState {
    pub container_telemetry: Mutex<ContainerTelemetry>,
    pub pocketqube_telemetry: Mutex<PocketQubeTelemetry>,
}
