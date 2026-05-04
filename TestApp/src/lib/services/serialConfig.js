import { invoke } from '@tauri-apps/api/core';

export async function listSerialPorts() {
  return await invoke('list_serial_ports');
}

export async function getSerialConfig() {
  return await invoke('get_serial_config');
}

/**
 * @param {string} port
 * @param {number} baudRate
 */
export async function setSerialConfig(port, baudRate) {
  return await invoke('set_serial_config', {
    port,
    baudRate
  });
}