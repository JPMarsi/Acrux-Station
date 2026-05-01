use super::telemetry::Telemetry;

pub fn parse_telemetry_line(line: &str) -> Option<Telemetry> {
    let clean = line.trim();

    let data_part = clean.split(",,").next()?;
    let fields: Vec<&str> = data_part.split(',').collect();

    if fields.len() < 22 {
        return None;
    }

    Some(Telemetry {
        team_id: fields[0].to_string(),
        mission_time: fields[1].to_string(),
        packet_count: fields[2].parse().ok()?,
        mode: fields[3].to_string(),
        state: fields[4].to_string(),
        altitude: fields[5].parse().ok()?,
        temperature: fields[6].parse().ok()?,
        pressure: fields[7].parse().ok()?,
        voltage: fields[8].parse().ok()?,
        current: fields[9].parse().ok()?,
        gyro_r: fields[10].parse().ok()?,
        gyro_p: fields[11].parse().ok()?,
        gyro_y: fields[12].parse().ok()?,
        accel_r: fields[13].parse().ok()?,
        accel_p: fields[14].parse().ok()?,
        accel_y: fields[15].parse().ok()?,
        gps_time: fields[16].to_string(),
        gps_altitude: fields[17].parse().ok()?,
        gps_latitude: fields[18].parse().ok()?,
        gps_longitude: fields[19].parse().ok()?,
        gps_sats: fields[20].parse().ok()?,
        cmd_echo: fields[21].to_string(),
        optional_data: if fields.len() > 22 {
            fields[22].to_string()
        } else {
            String::new()
        },
    })
}