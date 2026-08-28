import { writable } from 'svelte/store';

export const csvRecording = writable({
  active: false
});
