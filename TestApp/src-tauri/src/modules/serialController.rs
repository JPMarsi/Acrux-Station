use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use std::io::{ErrorKind, Read, Write};
use std::sync::{LazyLock, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkKind {
    EspNow,
    XBee,
}

impl LinkKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::EspNow => "ESP-NOW",
            Self::XBee => "XBEE",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    pub espnow_port: String,
    pub espnow_baud_rate: u32,
    pub xbee_port: String,
    pub xbee_baud_rate: u32,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            espnow_port: "COM6".into(),
            espnow_baud_rate: 115_200,
            xbee_port: "COM7".into(),
            xbee_baud_rate: 115_200,
        }
    }
}

struct SerialConnection {
    port: Box<dyn SerialPort>,
    line_buffer: Vec<u8>,
}

#[derive(Default)]
struct SerialPorts {
    espnow: Option<SerialConnection>,
    xbee: Option<SerialConnection>,
}

static SERIAL_CONFIG: LazyLock<Mutex<SerialConfig>> =
    LazyLock::new(|| Mutex::new(SerialConfig::default()));
static SERIAL_PORTS: LazyLock<Mutex<SerialPorts>> =
    LazyLock::new(|| Mutex::new(SerialPorts::default()));
static PREFERRED_COMMAND_LINK: LazyLock<Mutex<LinkKind>> =
    LazyLock::new(|| Mutex::new(LinkKind::EspNow));

fn connection_mut(ports: &mut SerialPorts, link: LinkKind) -> &mut Option<SerialConnection> {
    match link {
        LinkKind::EspNow => &mut ports.espnow,
        LinkKind::XBee => &mut ports.xbee,
    }
}

fn configured_endpoint(config: &SerialConfig, link: LinkKind) -> (&str, u32) {
    match link {
        LinkKind::EspNow => (&config.espnow_port, config.espnow_baud_rate),
        LinkKind::XBee => (&config.xbee_port, config.xbee_baud_rate),
    }
}

pub fn serial_init_link(link: LinkKind) -> Result<(), String> {
    if serial_is_open(link) {
        return Ok(());
    }
    let config = get_serial_config();
    let (port_path, baud) = configured_endpoint(&config, link);
    if port_path.trim().is_empty() {
        return Err(format!("Puerto {} no configurado", link.label()));
    }
    let port = serialport::new(port_path, baud)
        .timeout(Duration::from_millis(8))
        .open()
        .map_err(|e| format!("No se pudo abrir {} a {}: {}", port_path, baud, e))?;
    let mut ports = SERIAL_PORTS.lock().map_err(|e| e.to_string())?;
    *connection_mut(&mut ports, link) = Some(SerialConnection {
        port,
        line_buffer: Vec::with_capacity(512),
    });
    Ok(())
}

pub fn serial_close_link(link: LinkKind) {
    if let Ok(mut ports) = SERIAL_PORTS.lock() {
        *connection_mut(&mut ports, link) = None;
    }
}

pub fn serial_close_all() {
    if let Ok(mut ports) = SERIAL_PORTS.lock() {
        ports.espnow = None;
        ports.xbee = None;
    }
}

pub fn serial_is_open(link: LinkKind) -> bool {
    SERIAL_PORTS
        .lock()
        .map(|ports| match link {
            LinkKind::EspNow => ports.espnow.is_some(),
            LinkKind::XBee => ports.xbee.is_some(),
        })
        .unwrap_or(false)
}

pub fn serial_read_lines(link: LinkKind) -> Result<Vec<String>, String> {
    let mut ports = SERIAL_PORTS.lock().map_err(|e| e.to_string())?;
    let slot = connection_mut(&mut ports, link);
    let Some(connection) = slot.as_mut() else {
        return Ok(Vec::new());
    };
    let mut incoming = [0_u8; 256];
    loop {
        match connection.port.read(&mut incoming) {
            Ok(0) => break,
            Ok(count) => connection.line_buffer.extend_from_slice(&incoming[..count]),
            Err(error) if matches!(error.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock) => {
                break
            }
            Err(error) => {
                *slot = None;
                return Err(format!("{} desconectado: {}", link.label(), error));
            }
        }
    }
    let mut lines = Vec::new();
    while let Some(end) = connection
        .line_buffer
        .iter()
        .position(|byte| *byte == b'\n' || *byte == b'\r')
    {
        let raw: Vec<u8> = connection.line_buffer.drain(..end).collect();
        while matches!(connection.line_buffer.first(), Some(b'\n' | b'\r')) {
            connection.line_buffer.remove(0);
        }
        let line = String::from_utf8_lossy(&raw).trim().to_string();
        if !line.is_empty() {
            lines.push(line);
        }
    }
    if connection.line_buffer.len() > 16_384 {
        connection.line_buffer.clear();
        return Err(format!(
            "Buffer {} descartado por falta de fin de línea",
            link.label()
        ));
    }
    Ok(lines)
}

pub fn serial_write_link(link: LinkKind, line: &str) -> Result<(), String> {
    let mut ports = SERIAL_PORTS.lock().map_err(|e| e.to_string())?;
    let slot = connection_mut(&mut ports, link);
    let Some(connection) = slot.as_mut() else {
        return Err(format!("Puerto {} no inicializado", link.label()));
    };
    let payload = format!("{}\r\n", line.trim());
    if let Err(error) = connection
        .port
        .write_all(payload.as_bytes())
        .and_then(|_| connection.port.flush())
    {
        *slot = None;
        return Err(format!(
            "No se pudo escribir por {}: {}",
            link.label(),
            error
        ));
    }
    Ok(())
}

pub fn serial_write_preferred(line: &str) -> Result<LinkKind, String> {
    let preferred = *PREFERRED_COMMAND_LINK.lock().map_err(|e| e.to_string())?;
    let alternate = match preferred {
        LinkKind::EspNow => LinkKind::XBee,
        LinkKind::XBee => LinkKind::EspNow,
    };
    let mut errors = Vec::new();
    for link in [preferred, alternate] {
        if serial_is_open(link) {
            match serial_write_link(link, line) {
                Ok(()) => return Ok(link),
                Err(error) => errors.push(error),
            }
        }
    }
    if errors.is_empty() {
        Err("No hay ningún enlace serial disponible".into())
    } else {
        Err(errors.join(" | "))
    }
}

pub fn set_preferred_command_link(link: LinkKind) {
    if let Ok(mut preferred) = PREFERRED_COMMAND_LINK.lock() {
        *preferred = link;
    }
}

pub fn get_serial_config() -> SerialConfig {
    SERIAL_CONFIG.lock().unwrap().clone()
}

pub fn set_serial_config(config: SerialConfig) -> Result<SerialConfig, String> {
    if config.espnow_port == config.xbee_port && !config.espnow_port.trim().is_empty() {
        return Err("ESP-NOW y XBee deben usar puertos diferentes".into());
    }
    if config.espnow_baud_rate == 0 || config.xbee_baud_rate == 0 {
        return Err("El baudrate debe ser mayor que cero".into());
    }
    serial_close_all();
    *SERIAL_CONFIG.lock().map_err(|e| e.to_string())? = config.clone();
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::{LinkKind, SerialConfig};

    #[test]
    fn link_labels_are_unambiguous() {
        assert_eq!(LinkKind::EspNow.label(), "ESP-NOW");
        assert_eq!(LinkKind::XBee.label(), "XBEE");
    }

    #[test]
    fn default_ports_are_distinct() {
        let config = SerialConfig::default();
        assert_ne!(config.espnow_port, config.xbee_port);
    }
}
