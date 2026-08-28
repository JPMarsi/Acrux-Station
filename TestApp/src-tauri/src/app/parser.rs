use super::container_telemetry::ContainerTelemetry;
use super::pocketqube_telemetry::PocketQubeTelemetry;

pub const TEAM_ID: &str = "1234";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetrySource {
    Container,
    PocketQube,
}

/// Clasifica por el sufijo C/P y rechaza paquetes de otros equipos.
pub fn identify_telemetry_source(line: &str) -> Option<TelemetrySource> {
    match line.trim().split(',').next()? {
        id if id == format!("{TEAM_ID}C") => Some(TelemetrySource::Container),
        id if id == format!("{TEAM_ID}P") => Some(TelemetrySource::PocketQube),
        _ => None,
    }
}

fn telemetry_fields(line: &str) -> Vec<&str> {
    line.trim()
        .split(",,")
        .next()
        .unwrap_or_default()
        .split(',')
        .collect()
}

/// Parsea exclusivamente el paquete oficial de 13 campos del Container.
pub fn parse_container_telemetry(line: &str) -> Option<ContainerTelemetry> {
    if identify_telemetry_source(line)? != TelemetrySource::Container {
        return None;
    }
    let fields = telemetry_fields(line);
    if fields.len() != 13 {
        return None;
    }

    Some(ContainerTelemetry {
        id: fields[0].into(),
        mission_time: fields[1].into(),
        packet_count: fields[2].parse().ok()?,
        command_count: fields[3].parse().ok()?,
        mode: fields[4].into(),
        altitude: fields[5].parse().ok()?,
        pressure: fields[6].parse().ok()?,
        temperature: fields[7].parse().ok()?,
        batt_v: fields[8].parse().ok()?,
        batt_i: fields[9].parse().ok()?,
        mech_state: fields[10].into(),
        state: fields[11].into(),
        cmd_echo: fields[12].into(),
    })
}

/// Parsea los 30 campos base. Campos científicos adicionales se conservan
/// dentro de SCIENCE_EXP para no perder datos opcionales del equipo.
pub fn parse_pocketqube_telemetry(line: &str) -> Option<PocketQubeTelemetry> {
    if identify_telemetry_source(line)? != TelemetrySource::PocketQube {
        return None;
    }
    let fields = telemetry_fields(line);
    if fields.len() < 30 {
        return None;
    }

    Some(PocketQubeTelemetry {
        id: fields[0].into(),
        mode: fields[1].into(),
        mission_time: fields[2].into(),
        packet_count: fields[3].parse().ok()?,
        command_count: fields[4].parse().ok()?,
        altitude: fields[5].parse().ok()?,
        temperature: fields[6].parse().ok()?,
        pressure: fields[7].parse().ok()?,
        voltage: fields[8].parse().ok()?,
        current: fields[9].parse().ok()?,
        gnss_time: fields[10].into(),
        gnss_altitude: fields[11].parse().ok()?,
        gnss_latitude: fields[12].parse().ok()?,
        gnss_longitude: fields[13].parse().ok()?,
        gnss_sats: fields[14].parse().ok()?,
        rot_rate_x: fields[15].parse().ok()?,
        rot_rate_y: fields[16].parse().ok()?,
        rot_rate_z: fields[17].parse().ok()?,
        accel_x: fields[18].parse().ok()?,
        accel_y: fields[19].parse().ok()?,
        accel_z: fields[20].parse().ok()?,
        mag_x: fields[21].parse().ok()?,
        mag_y: fields[22].parse().ok()?,
        mag_z: fields[23].parse().ok()?,
        solar_1: fields[24].parse().ok()?,
        solar_2: fields[25].parse().ok()?,
        mech_state: fields[26].into(),
        cmd_echo: fields[27].into(),
        image_stabilization: fields[28].into(),
        science_exp: fields[29..].join(" | "),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTAINER: &str = "1234C,1.001,7,2,F,100.1,101325,24.5,8.2,125,0x03,PQ_RELEASE,CXON";
    const POCKETQUBE: &str = "1234P,F,2.0,8,3,90.2,23.1,101200,7.9,110,12:00:02,92.0,-33.2,-66.3,9,1.1,2.2,3.3,0.1,0.2,9.8,10,20,30,4.5,4.6,0x0F,MEC,STABLE,NONE";

    #[test]
    fn parses_official_container_packet() {
        let parsed = parse_container_telemetry(CONTAINER).unwrap();
        assert_eq!(parsed.id, "1234C");
        assert_eq!(parsed.command_count, 2);
        assert_eq!(parsed.state, "PQ_RELEASE");
    }

    #[test]
    fn parses_official_pocketqube_packet() {
        let parsed = parse_pocketqube_telemetry(POCKETQUBE).unwrap();
        assert_eq!(parsed.id, "1234P");
        assert_eq!(parsed.gnss_sats, 9);
        assert_eq!(parsed.solar_2, 4.6);
    }

    #[test]
    fn rejects_wrong_team_crossed_parser_and_wrong_length() {
        assert!(identify_telemetry_source(&CONTAINER.replacen("1234", "9999", 1)).is_none());
        assert!(parse_container_telemetry(POCKETQUBE).is_none());
        assert!(parse_pocketqube_telemetry(CONTAINER).is_none());
        assert!(parse_container_telemetry("1234C,1,2").is_none());
    }

    #[test]
    fn keeps_additional_science_fields() {
        let parsed = parse_pocketqube_telemetry(&format!("{POCKETQUBE},VALUE_2")).unwrap();
        assert_eq!(parsed.science_exp, "NONE | VALUE_2");
    }
}
