import { writable } from 'svelte/store';

const MAX_POINTS = 20;

/** @typedef {{
 * labels: number[],
 * altitude: number[],
 * temperature: number[],
 * pressure: number[],
 * voltage: number[],
 * current: number[],
 * gps_altitude: number[]
 * }} TelemetryHistory
 */

/** @type {TelemetryHistory} */
const initialHistory = {
  labels: [],
  altitude: [],
  temperature: [],
  pressure: [],
  voltage: [],
  current: [],
  gps_altitude: []
};

/**
 * @param {number[]} array
 * @param {number} value
 * @param {number} [max]
 */
function pushLimited(array, value, max = MAX_POINTS) {
  const next = [...array, Number(value ?? 0)];
  return next.slice(-max);
}

function createTelemetryHistory() {
  /** @type {import('svelte/store').Writable<TelemetryHistory>} */
  const store = writable(initialHistory);
  const { subscribe, update, set } = store;

  return {
    subscribe,

    reset() {
      set(initialHistory);
    },

    /**
     * @param {{
     * packet_count?: number,
     * altitude?: number,
     * temperature?: number,
     * pressure?: number,
     * voltage?: number,
     * current?: number,
     * gps_altitude?: number
     * }} sample
     */
    append(sample) {
      update((history) => ({
        labels: pushLimited(
          history.labels,
          sample.packet_count ?? history.labels.length + 1
        ),
        altitude: pushLimited(history.altitude, sample.altitude ?? 0),
        temperature: pushLimited(history.temperature, sample.temperature ?? 0),
        pressure: pushLimited(history.pressure, sample.pressure ?? 0),
        voltage: pushLimited(history.voltage, sample.voltage ?? 0),
        current: pushLimited(history.current, sample.current ?? 0),
        gps_altitude: pushLimited(history.gps_altitude, sample.gps_altitude ?? 0)
      }));
    }
  };
}

export const telemetryHistory = createTelemetryHistory();