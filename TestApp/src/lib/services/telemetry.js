import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { telemetry } from '../stores/telemetry';
import { telemetryHistory } from '../stores/telemetryHistory';

/** @type {null | (() => void)} */
let unlisten = null;

export async function initTelemetry() {
  try {
    const initialData = await invoke('get_telemetry');
    telemetry.set(initialData);
    telemetryHistory.append(initialData);
  } catch (error) {
    console.error('Error loading initial telemetry:', error);
  }

  unlisten = await listen('telemetry-update', (event) => {
    const sample = event.payload;
    telemetry.set(sample);
    telemetryHistory.append(sample);
  });
}

export async function resetTelemetrySession() {
  const resetData = await invoke('reset_app');
  telemetry.set(resetData);
  telemetryHistory.reset();
  return resetData;
}

export function destroyTelemetryListener() {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
}
