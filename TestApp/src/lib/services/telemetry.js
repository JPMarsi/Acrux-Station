import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { containerTelemetry, pocketQubeTelemetry } from '../stores/telemetry';
import { containerTelemetryHistory, pocketQubeTelemetryHistory } from '../stores/telemetryHistory';
import { csvRecording } from '../stores/csvRecording';
import { linkStatus } from '../stores/linkStatus';

/** @type {null | (() => void)} */
let unlistenContainer = null;
/** @type {null | (() => void)} */
let unlistenPocketQube = null;
/** @type {null | (() => void)} */
let unlistenCsvRecording = null;
/** @type {null | (() => void)} */
let unlistenLinkStatus = null;

export async function initTelemetry() {
  try {
    const initialData = await invoke('get_telemetry');
    containerTelemetry.set(initialData.container);
    pocketQubeTelemetry.set(initialData.pocketqube);
    const initialCsvStatus = await invoke('get_csv_recording_status');
    csvRecording.set(initialCsvStatus);
  } catch (error) {
    console.error('Error loading initial telemetry:', error);
  }

  unlistenContainer = await listen('container-telemetry-update', (event) => {
    const sample = event.payload;
    containerTelemetry.set(sample);
    containerTelemetryHistory.append(sample);
  });

  unlistenPocketQube = await listen('pocketqube-telemetry-update', (event) => {
    const sample = event.payload;
    pocketQubeTelemetry.set(sample);
    pocketQubeTelemetryHistory.append(sample);
  });

  unlistenCsvRecording = await listen('csv-recording-update', (event) => {
    csvRecording.set(event.payload);
  });

  unlistenLinkStatus = await listen('link-status-update', (event) => {
    linkStatus.set(event.payload);
  });

}

export async function resetTelemetrySession() {
  const resetData = await invoke('reset_app');
  containerTelemetry.set(resetData.container);
  pocketQubeTelemetry.set(resetData.pocketqube);
  containerTelemetryHistory.reset();
  pocketQubeTelemetryHistory.reset();
  return resetData;
}

export function destroyTelemetryListener() {
  if (unlistenContainer) unlistenContainer();
  if (unlistenPocketQube) unlistenPocketQube();
  if (unlistenCsvRecording) unlistenCsvRecording();
  if (unlistenLinkStatus) unlistenLinkStatus();
  unlistenContainer = null;
  unlistenPocketQube = null;
  unlistenCsvRecording = null;
  unlistenLinkStatus = null;
}
