import { writable } from 'svelte/store';

export const linkStatus = writable({
  espnow_connected: false,
  xbee_connected: false,
  container_source: 'SIN DATOS',
  pocketqube_source: 'SIN DATOS',
  espnow_last_packet_ms: null,
  xbee_last_packet_ms: null
});
