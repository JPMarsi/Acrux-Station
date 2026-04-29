import { writable } from 'svelte/store';

export const telemetry = writable({
  team_id: '----',
  mission_time: '--:--:--',
  packet_count: 0,
  mode: '----',
  state: '----',
  altitude: 0,
  temperature: 0,
  pressure: 0,
  voltage: 0,
  current: 0,
  gyro_r: 0,
  gyro_p: 0,
  gyro_y: 0,
  accel_r: 0,
  accel_p: 0,
  accel_y: 0,
  gps_time: '--:--:--',
  gps_altitude: 0,
  gps_latitude: 0,
  gps_longitude: 0,
  gps_sats: 0,
  cmd_echo: '----',
  optional_data: '----'
});