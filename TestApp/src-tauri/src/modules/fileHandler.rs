use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

// =========================
// VARIABLES GLOBALES
// =========================

lazy_static::lazy_static! {
    pub static ref var_text_from_serial_to_json_telemetry: Mutex<Option<String>> = Mutex::new(None);
    pub static ref var_text_from_csv_to_json_telemetry: Mutex<Option<String>> = Mutex::new(None);

    static ref BASE_PATH: Mutex<String> = Mutex::new("./data".to_string());

    static ref TELEMETRY_FILES: Mutex<HashMap<u32, String>> = Mutex::new(HashMap::new());
    static ref TELEMETRY_RECORDING_ACTIVE: Mutex<bool> = Mutex::new(false);
}

pub const CONTAINER_FILE_ID: u32 = 1;
pub const POCKETQUBE_FILE_ID: u32 = 2;
pub const CONTAINER_HEADER: &str = "ID,MISSION_TIME,PACKET_COUNT,COMMAND_COUNT,MODE,ALTITUDE,PRESSURE,TEMPERATURE,BATT_V,BATT_I,MECH_STATE,STATE,CMD_ECHO";
pub const POCKETQUBE_HEADER: &str = "ID,MODE,MISSION_TIME,PACKET_COUNT,COMMAND_COUNT,ALTITUDE,TEMPERATURE,PRESSURE,VOLTAGE,CURRENT,GNSS_TIME,GNSS_ALTITUDE,GNSS_LATITUDE,GNSS_LONGITUDE,GNSS_SATS,ROT_RATE_X,ROT_RATE_Y,ROT_RATE_Z,ACCEL_X,ACCEL_Y,ACCEL_Z,MAG_X,MAG_Y,MAG_Z,SOLAR_1,SOLAR_2,MECH_STATE,CMD_ECHO,IMAGE_STABILIZATION,SCIENCE_EXP";

// =========================
// PATH
// =========================

pub fn file_path(p: &str) {
    set_csv_output_directory(p).unwrap();
}

pub fn set_csv_output_directory(p: &str) -> Result<String, String> {
    let path = PathBuf::from(p);
    if !path.exists() {
        fs::create_dir_all(&path)
            .map_err(|e| format!("No se pudo crear {}: {}", path.display(), e))?;
    }
    if !path.is_dir() {
        return Err(format!("{} no es una carpeta", path.display()));
    }

    let normalized = path.to_string_lossy().to_string();
    *BASE_PATH.lock().map_err(|e| e.to_string())? = normalized.clone();
    Ok(normalized)
}

// =========================
// TIME
// =========================

pub fn timestamp() -> String {
    use chrono::{Datelike, Local, Timelike};

    let d = Local::now();

    let Y = d.year();
    let M = format!("{:02}", d.month());
    let D = format!("{:02}", d.day());

    let h = format!("{:02}", d.hour());
    let m = format!("{:02}", d.minute());
    let s = format!("{:02}", d.second());

    format!("{}_{}_{}_{}_{}_{}", Y, M, D, h, m, s)
}

// =========================
// JSON -> CSV
// =========================

pub fn data_getHandle_from_json_to_csv(data: &HashMap<String, String>) -> Option<String> {
    if data.is_empty() {
        return None;
    }

    let values: Vec<String> = data.values().cloned().collect();
    Some(values.join(","))
}

// =========================
// JSON -> TXT
// =========================

pub fn data_get_from_json_to_txt(data: &HashMap<String, String>, format: &str) -> Option<String> {
    if data.is_empty() {
        return None;
    }

    let mut out: Vec<String> = Vec::new();

    if format == "var" {
        for (k, v) in data {
            out.push(format!("{} = {}", k, v));
        }
    }

    if format == "cmd" || format == "msg" {
        for (k, v) in data {
            out.push(format!("{} {}", k, v));
        }
    }

    Some(out.join("\n"))
}

// =========================
// CSV GENERICO
// =========================

pub fn file_csv_create(name: &str) -> String {
    let base = BASE_PATH.lock().unwrap();
    let mut full = PathBuf::from(base.as_str());
    full.push(name);

    fs::write(&full, "").unwrap();

    full.to_string_lossy().to_string()
}

// =========================
// CSV TELEMETRY
// =========================

pub fn file_csv_create_telemetry(id: u32, team: u32) -> String {
    let name = format!("telemetry_team{}_id{:04}_{}.csv", team, id, timestamp());

    let base = BASE_PATH.lock().unwrap();
    let mut full = PathBuf::from(base.as_str());
    full.push(&name);

    fs::write(&full, "").unwrap();

    TELEMETRY_FILES
        .lock()
        .unwrap()
        .insert(id, full.to_string_lossy().to_string());

    full.to_string_lossy().to_string()
}

pub fn file_csv_create_flight_telemetry(id: u32, team: u32) -> String {
    let name = format!("Flight_{}.csv", team);

    let base = BASE_PATH.lock().unwrap();
    let mut full = PathBuf::from(base.as_str());
    full.push(&name);

    fs::write(&full, "").unwrap();

    TELEMETRY_FILES
        .lock()
        .unwrap()
        .insert(id, full.to_string_lossy().to_string());

    full.to_string_lossy().to_string()
}

pub fn official_flight_file_name(team: u32, payload_suffix: char) -> String {
    format!("Flight_{}{}.csv", team, payload_suffix)
}

fn create_official_flight_file(id: u32, team: u32, suffix: char) -> Result<String, String> {
    let name = official_flight_file_name(team, suffix);
    let base = BASE_PATH.lock().map_err(|e| e.to_string())?;
    let full = PathBuf::from(base.as_str()).join(name);

    fs::write(&full, "").map_err(|e| format!("No se pudo crear {}: {}", full.display(), e))?;
    TELEMETRY_FILES
        .lock()
        .map_err(|e| e.to_string())?
        .insert(id, full.to_string_lossy().to_string());

    Ok(full.to_string_lossy().to_string())
}

/// Inicia una mision creando simultaneamente los dos CSV oficiales 2027.
pub fn file_csv_start_official_flight_recording(team: u32) -> Result<(String, String), String> {
    if file_csv_is_recording() {
        return Err("Ya existe una grabación CSV activa; detenela antes de iniciar otra".into());
    }

    let container = create_official_flight_file(CONTAINER_FILE_ID, team, 'C')?;
    let pocketqube = create_official_flight_file(POCKETQUBE_FILE_ID, team, 'P')?;

    file_csv_writeHeader_telemetry(CONTAINER_FILE_ID, CONTAINER_HEADER);
    file_csv_writeHeader_telemetry(POCKETQUBE_FILE_ID, POCKETQUBE_HEADER);
    *TELEMETRY_RECORDING_ACTIVE
        .lock()
        .map_err(|e| e.to_string())? = true;

    Ok((container, pocketqube))
}

pub fn file_csv_start_recording(id: u32, team: u32, format: &str) -> Result<String, String> {
    file_path("./data");

    let file = match format.to_ascii_lowercase().as_str() {
        "session" | "local" | "timestamp" => file_csv_create_telemetry(id, team),
        "mission" | "flight" => file_csv_create_flight_telemetry(id, team),
        _ => {
            return Err(format!(
                "Formato CSV no reconocido: {}. Usar SESSION o MISSION",
                format
            ));
        }
    };

    let mut active = TELEMETRY_RECORDING_ACTIVE.lock().unwrap();
    *active = true;

    Ok(file)
}

pub fn file_csv_stop_recording() {
    let mut active = TELEMETRY_RECORDING_ACTIVE.lock().unwrap();
    *active = false;
}

pub fn file_csv_is_recording() -> bool {
    *TELEMETRY_RECORDING_ACTIVE.lock().unwrap()
}

pub fn file_csv_writeHeader_telemetry(id: u32, header: &str) {
    let files = TELEMETRY_FILES.lock().unwrap();
    let file = match files.get(&id) {
        Some(f) => f,
        None => return,
    };

    let mut content = header.to_string();
    content.push_str("\r\n");

    fs::OpenOptions::new()
        .append(true)
        .open(file)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

pub fn file_csv_writeLine_telemetry(id: u32, line: &str) {
    let files = TELEMETRY_FILES.lock().unwrap();
    let file = match files.get(&id) {
        Some(f) => f,
        None => return,
    };

    let mut content = line.to_string();
    content.push_str("\r\n");

    let _ = fs::OpenOptions::new()
        .append(true)
        .open(file)
        .and_then(|mut output| output.write_all(content.as_bytes()));
}

pub fn file_csv_writeLine_telemetry_if_recording(id: u32, line: &str) {
    if file_csv_is_recording() {
        file_csv_writeLine_telemetry(id, line);
    }
}

use std::io::Write;

pub fn file_csv_readLine_telemetry(id: u32, line: usize) -> Option<String> {
    let files = TELEMETRY_FILES.lock().unwrap();
    let file = files.get(&id)?;

    let content = fs::read_to_string(file).ok()?;
    let lines: Vec<&str> = content.lines().collect();

    lines.get(line).map(|s| s.to_string())
}

// --- TESTING ---

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::Path;

    fn setup() {
        let test_dir = "./test_data";
        file_path(test_dir);
    }

    fn cleanup() {
        std::fs::remove_dir_all("./test_data").ok();
    }

    #[test]
    fn test_print() {
        println!("print test");
    }

    #[test]
    fn test_json_to_csv() {
        let mut data = HashMap::new();
        data.insert("a".to_string(), "1".to_string());
        data.insert("b".to_string(), "2".to_string());

        let csv = data_getHandle_from_json_to_csv(&data).unwrap();

        assert!(csv.contains("1"));
        assert!(csv.contains("2"));
    }

    #[test]
    fn test_json_to_txt_var() {
        let mut data = HashMap::new();
        data.insert("temp".to_string(), "25".to_string());

        let txt = data_get_from_json_to_txt(&data, "var").unwrap();

        assert_eq!(txt, "temp = 25");
    }

    #[test]
    fn test_json_to_txt_cmd() {
        let mut data = HashMap::new();
        data.insert("LED".to_string(), "ON".to_string());

        let txt = data_get_from_json_to_txt(&data, "cmd").unwrap();

        assert_eq!(txt, "LED ON");
    }

    #[test]
    fn test_file_create() {
        setup();

        let file = file_csv_create("test.csv");

        assert!(Path::new(&file).exists());

        cleanup();
    }

    #[test]
    fn test_create_telemetry_file() {
        setup();

        let file = file_csv_create_telemetry(1, 2);

        assert!(Path::new(&file).exists());

        cleanup();
    }

    #[test]
    fn test_write_and_read() {
        setup();

        let id = 10;

        file_csv_create_telemetry(id, 1);

        file_csv_writeHeader_telemetry(id, "A,B,C");
        file_csv_writeLine_telemetry(id, "1,2,3");

        let line0 = file_csv_readLine_telemetry(id, 0).unwrap();
        let line1 = file_csv_readLine_telemetry(id, 1).unwrap();

        assert_eq!(line0, "A,B,C");
        assert_eq!(line1, "1,2,3");

        cleanup();
    }

    #[test]
    fn test_timestamp_format() {
        let ts = timestamp();

        assert_eq!(ts.len(), 19);
        assert!(ts.contains("_"));
    }

    #[test]
    fn official_names_and_headers_match_2027_formats() {
        assert_eq!(official_flight_file_name(1234, 'C'), "Flight_1234C.csv");
        assert_eq!(official_flight_file_name(1234, 'P'), "Flight_1234P.csv");
        assert_eq!(CONTAINER_HEADER.split(',').count(), 13);
        assert_eq!(POCKETQUBE_HEADER.split(',').count(), 30);
    }
}
