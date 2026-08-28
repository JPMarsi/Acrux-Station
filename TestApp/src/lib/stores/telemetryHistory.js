import { writable } from 'svelte/store';

const MAX_POINTS = 20;

/** @param {number[]} array @param {unknown} value */
function pushLimited(array, value) {
  return [...array, Number(value ?? 0)].slice(-MAX_POINTS);
}

function createContainerHistory() {
  /** @type {{labels:number[], altitude:number[], temperature:number[], batt_i:number[]}} */
  const initial = { labels: [], altitude: [], temperature: [], batt_i: [] };
  const { subscribe, update, set } = writable(initial);
  return {
    subscribe,
    reset: () => set(initial),
    /** @param {any} sample */
    append: (sample) => update((history) => ({
      labels: pushLimited(history.labels, sample.packet_count),
      altitude: pushLimited(history.altitude, sample.altitude),
      temperature: pushLimited(history.temperature, sample.temperature),
      batt_i: pushLimited(history.batt_i, sample.batt_i)
    }))
  };
}

function createPocketQubeHistory() {
  /** @type {{labels:number[], altitude:number[], temperature:number[], voltage:number[], current:number[], rot_rate_x:number[], rot_rate_y:number[], rot_rate_z:number[], gnss_latitude:number[], gnss_longitude:number[], solar_1:number[], solar_2:number[]}} */
  const initial = {
    labels: [], altitude: [], temperature: [], voltage: [], current: [],
    rot_rate_x: [], rot_rate_y: [], rot_rate_z: [], gnss_latitude: [], gnss_longitude: [],
    solar_1: [], solar_2: []
  };
  const { subscribe, update, set } = writable(initial);
  return {
    subscribe,
    reset: () => set(initial),
    /** @param {any} sample */
    append: (sample) => update((history) => ({
      labels: pushLimited(history.labels, sample.packet_count),
      altitude: pushLimited(history.altitude, sample.altitude),
      temperature: pushLimited(history.temperature, sample.temperature),
      voltage: pushLimited(history.voltage, sample.voltage),
      current: pushLimited(history.current, sample.current),
      rot_rate_x: pushLimited(history.rot_rate_x, sample.rot_rate_x),
      rot_rate_y: pushLimited(history.rot_rate_y, sample.rot_rate_y),
      rot_rate_z: pushLimited(history.rot_rate_z, sample.rot_rate_z),
      gnss_latitude: pushLimited(history.gnss_latitude, sample.gnss_latitude),
      gnss_longitude: pushLimited(history.gnss_longitude, sample.gnss_longitude),
      solar_1: pushLimited(history.solar_1, sample.solar_1),
      solar_2: pushLimited(history.solar_2, sample.solar_2)
    }))
  };
}

export const containerTelemetryHistory = createContainerHistory();
export const pocketQubeTelemetryHistory = createPocketQubeHistory();
