/* #-# --- -.- - - . . -*- . . - - -.- --- #-# */
/* #-# --- -.- - - 00glitched  - - -.- --- #-# */
/* #-# --- -.- - - . . -*- . . - - -.- --- #-# */

// serialController.rs
// manejo del puerto serie y parsing de telemetría

use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
pub struct SerialConfig {
    pub selected_port: String,
    pub baud_rate: u32,
}

static SERIAL_CONFIG: Mutex<Option<SerialConfig>> = Mutex::new(None);
// puerto y buffer globales (similar a JS)
static mut PORT: Option<Box<dyn SerialPort>> = None;

// variable exportable con la linea cruda de telemetria
pub static mut VAR_TEXT_FROM_SERIAL_TO_RAW_TELEMETRY: Option<String> = None;

/*
// serial_init(puerto, baudios)
// abre el puerto serie y prepara lectura por linea
*/
pub fn serial_init(port_path: &str, baud: u32) -> Result<(), Box<dyn std::error::Error>> {
    let port = serialport::new(port_path, baud)
        .timeout(Duration::from_millis(10))
        .open()?;

    unsafe {
        PORT = Some(port);
    }

    Ok(())
}

pub fn serial_close() {
    unsafe {
        PORT = None;
        VAR_TEXT_FROM_SERIAL_TO_RAW_TELEMETRY = None;
    }
}

/* --- INPUTS --- */

/*
// serial_detect(line)
// detecta telemetria buscando ",,telemetry"
*/
pub fn serial_detect(line: &str) -> Option<&'static str> {
    let clean = line.trim();

    if clean.contains(",,telemetry") {
        unsafe {
            VAR_TEXT_FROM_SERIAL_TO_RAW_TELEMETRY = Some(clean.to_string());
        }
        return Some("telemetry");
    }

    Some("unknown")
}

/*
// serial_read_to_json_telemetry()
// convierte la telemetria guardada a json
*/
use std::collections::HashMap;

pub fn serial_read_to_json_telemetry() -> Option<HashMap<String, String>> {
    let text;
    unsafe {
        text = VAR_TEXT_FROM_SERIAL_TO_RAW_TELEMETRY.clone()?;
    }

    let split: Vec<&str> = text.split(",,").collect();
    let data = split.get(0)?;

    let fields: Vec<&str> = data.split(',').collect();

    let mut json = HashMap::new();

    for (i, field) in fields.iter().enumerate() {
        json.insert(format!("FIELD_{}", i), field.to_string());
    }

    Some(json)
}

/* --- TIME --- */

/*
// serial_write_from_cmd_timeSetTime([hora,mins,segs])
*/
pub fn serial_write_from_cmd_time_set_time(team_id: u16, time: [u8; 3]) {
    unsafe {
        if let Some(port) = PORT.as_mut() {
            let cmd = format!("{:02}:{:02}:{:02}", time[0], time[1], time[2]);
            let message = format!("CMD,{},ST,{}\r\n", team_id, cmd);

            let _ = port.write(message.as_bytes());
        }
    }
}

/* --- TOGGLES --- */

/*
// serial_write_from_cmd_telemetry(equipo, estado)
*/
pub fn serial_write_from_cmd_telemetry(team_id: u16, state: bool) {
    unsafe {
        if let Some(port) = PORT.as_mut() {
            let cmd = if state { "ON" } else { "OFF" };
            let message = format!("CMD,{},CX,{}\r\n", team_id, cmd);

            let _ = port.write(message.as_bytes());
        }
    }
}

/*
// serial_write_from_cmd_mechanism(equipo, dispositivo, estado)
*/
pub fn serial_write_from_cmd_mechanism(team_id: u16, device: Option<u8>, state: bool) {
    unsafe {
        if let Some(port) = PORT.as_mut() {
            let dev = match device {
                Some(d) => d,
                None => return,
            };

            let cmd = if state { "ON" } else { "OFF" };
            let message = format!("CMD,{},MEC,{},{}\r\n", team_id, dev, cmd);

            let _ = port.write(message.as_bytes());
        }
    }
}

/*
// serial_write_from_cmd_altitudeSetZero(equipo)
*/
pub fn serial_write_from_cmd_altitude_set_zero(team_id: u16) {
    unsafe {
        if let Some(port) = PORT.as_mut() {
            let message = format!("CMD,{},CAL\r\n", team_id);
            let _ = port.write(message.as_bytes());
        }
    }
}

/* --- LECTURA POR LINEA --- */

pub fn serial_read_line() -> Option<String> {
    let mut buffer = [0u8; 1];
    let mut line = String::new();

    unsafe {
        let port = PORT.as_mut()?;

        loop {
            match port.read(&mut buffer) {
                Ok(1) => {
                    let c = buffer[0] as char;
                    line.push(c);

                    if line.ends_with("\r\n") {
                        return Some(line.trim().to_string());
                    }
                }
                _ => break,
            }
        }
    }

    None
}
//funcion general de escritura
pub fn serial_write_raw(line: &str) -> Result<(), String> {
    unsafe {
        if let Some(port) = PORT.as_mut() {
            use std::io::Write;

            let payload = format!("{}\n", line);
            port.write_all(payload.as_bytes())
                .map_err(|e| e.to_string())?;
            port.flush().map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Puerto serial no inicializado".into())
        }
    }
}

pub fn get_serial_config() -> SerialConfig {
    let config = SERIAL_CONFIG.lock().unwrap();

    config.clone().unwrap_or(SerialConfig {
        selected_port: "COM6".into(),
        baud_rate: 115200,
    })
}

pub fn set_serial_config(port: String, baud_rate: u32) -> Result<SerialConfig, String> {
    let new_config = SerialConfig {
        selected_port: port,
        baud_rate,
    };

    let mut config = SERIAL_CONFIG.lock().unwrap();
    *config = Some(new_config.clone());

    Ok(new_config)
}
