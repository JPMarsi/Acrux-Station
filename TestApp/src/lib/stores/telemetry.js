import { writable } from 'svelte/store';

// Estructura oficial 2027 exclusiva del Container.
export const containerTelemetry = writable({
  id: '1234C',
  mission_time: '0.000',
  packet_count: 0,
  command_count: 0,
  mode: 'F',
  altitude: 0,
  pressure: 0,
  temperature: 0,
  batt_v: 0,
  batt_i: 0,
  mech_state: '0x00',
  state: 'LAUNCH_PAD',
  cmd_echo: 'NONE'
});

// Estructura oficial 2027 exclusiva del PocketQube.
export const pocketQubeTelemetry = writable({
  id: '1234P',
  mode: 'F',
  mission_time: '0',
  packet_count: 0,
  command_count: 0,
  altitude: 0,
  temperature: 0,
  pressure: 0,
  voltage: 0,
  current: 0,
  gnss_time: '00:00:00',
  gnss_altitude: 0,
  gnss_latitude: 0,
  gnss_longitude: 0,
  gnss_sats: 0,
  rot_rate_x: 0,
  rot_rate_y: 0,
  rot_rate_z: 0,
  accel_x: 0,
  accel_y: 0,
  accel_z: 0,
  mag_x: 0,
  mag_y: 0,
  mag_z: 0,
  solar_1: 0,
  solar_2: 0,
  mech_state: '0x00',
  cmd_echo: 'NONE',
  image_stabilization: 'NONE',
  science_exp: 'NONE'
});
