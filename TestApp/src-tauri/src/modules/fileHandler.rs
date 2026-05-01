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
}

// =========================
// PATH
// =========================

pub fn file_path(p: &str) {
    let mut base = BASE_PATH.lock().unwrap();
    *base = p.to_string();

    let path = PathBuf::from(p);
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
}

// =========================
// TIME
// =========================

pub fn timestamp() -> String {
    use chrono::{Datelike, Timelike, Local};

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

pub fn data_get_from_json_to_txt(
    data: &HashMap<String, String>,
    format: &str,
) -> Option<String> {
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
    let name = format!(
        "telemetry_team{}_id{:04}_{}.csv",
        team,
        id,
        timestamp()
    );

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

pub fn file_csv_writeHeader_telemetry(id: u32, header: &str) {
    let files = TELEMETRY_FILES.lock().unwrap();
    let file = match files.get(&id) {
        Some(f) => f,
        None => return,
    };

    let mut content = header.to_string();
    content.push('\n');

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
    content.push('\n');

    fs::OpenOptions::new()
        .append(true)
        .open(file)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

use std::io::Write;

pub fn file_csv_readLine_telemetry(id: u32, line: usize) -> Option<String> {
    let files = TELEMETRY_FILES.lock().unwrap();
    let file = files.get(&id)?;

    let content = fs::read_to_string(file).ok()?;
    let lines: Vec<&str> = content.split('\n').collect();

    lines.get(line).map(|s| s.to_string())
}
