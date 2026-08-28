import { invoke } from '@tauri-apps/api/core';

export async function listSerialPorts() {
  return await invoke('list_serial_ports');
}

export async function getSerialConfig() {
  return await invoke('get_serial_config');
}

/**
 * @param {{espnow_port:string, espnow_baud_rate:number, xbee_port:string, xbee_baud_rate:number}} config
 */
export async function setSerialConfig(config) {
  return await invoke('set_serial_config', {
    espnowPort: config.espnow_port,
    espnowBaudRate: config.espnow_baud_rate,
    xbeePort: config.xbee_port,
    xbeeBaudRate: config.xbee_baud_rate
  });
}
