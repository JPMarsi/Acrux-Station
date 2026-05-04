import { writable } from 'svelte/store';

const MAX_HISTORY = 5;

/**
 * @typedef {'idle' | 'success' | 'error'} CommandLineType
 */

/**
 * @typedef {{
 * message: string,
 * type: CommandLineType
 * }} CommandLine
 */

/**
 * @typedef {{
 * lines: CommandLine[],
 * loading: boolean
 * }} CommandConsoleState
 */

function createCommandConsole() {
  /** @type {import('svelte/store').Writable<CommandConsoleState>} */
  const store = writable({
    lines: [
      {
        message: '[READY] Esperando comando...',
        type: 'idle'
      }
    ],
    loading: false
  });

  const { subscribe, update, set } = store;

  return {
    subscribe,

    /**
     * @param {boolean} loading
     */
    setLoading(loading) {
      update((state) => ({
        ...state,
        loading
      }));
    },

    /**
     * @param {string} message
     * @param {CommandLineType} [type]
     */
    push(message, type = 'idle') {
      update((state) => ({
        ...state,
        lines: [
          ...state.lines,
          { message, type }
        ].slice(-MAX_HISTORY)
      }));
    },

    reset() {
      set({
        lines: [
          {
            message: '[READY] Esperando comando...',
            type: 'idle'
          }
        ],
        loading: false
      });
    }
  };
}

export const commandConsole = createCommandConsole();