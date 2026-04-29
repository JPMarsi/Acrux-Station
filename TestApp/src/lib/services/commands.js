import { invoke } from '@tauri-apps/api/core';

/**
 * @param {string} command
 */
export async function sendCustomCommand(command) {
  return await invoke('send_custom_command', { command });
}