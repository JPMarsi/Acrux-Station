<script lang="ts">
  import { onMount } from 'svelte';
  import { executeProtocolCommand } from '../services/commandExecutor';
  import { commandConsole } from '../stores/commandConsole';
  import {
    listSerialPorts,
    getSerialConfig,
    setSerialConfig
  } from '../services/serialConfig';

  type CommandButton = {
    label: string;
    command: string;
  };

  const commandButtons: CommandButton[] = [
    { label: 'START TELEMETRY', command: 'CMD,1234,CX,ON' },
    { label: 'END TELEMETRY', command: 'CMD,1234,CX,OFF' },
    { label: 'CAL ALTITUDE', command: 'CMD,1234,CAL' },

    { label: 'SET TIME', command: '' },
    { label: 'CAL PYR', command: '' },
    { label: 'ENABLE SIM', command: '' },
    { label: 'ACTIVATE SIM', command: '' },
    { label: 'DEACTIVATE SIM', command: '' }
  ];

  const baudOptions = [9600, 19200, 38400, 57600, 115200];

  let availablePorts: string[] = [];
  let selectedPort = 'COM6';
  let selectedBaud = 115200;

  onMount(async () => {
    try {
      availablePorts = await listSerialPorts();

      const config = await getSerialConfig();
      selectedPort = config.selected_port;
      selectedBaud = config.baud_rate;
    } catch (error) {
      console.error('Error loading serial config:', error);
    }
  });

  async function handleCommand(command: string) {
    if (!command) return;
    await executeProtocolCommand(command);
  }

  async function handlePortChange() {
    try {
      await setSerialConfig(selectedPort, selectedBaud);
      commandConsole.push(`[OK] Puerto seleccionado: ${selectedPort}`, 'success');
    } catch (error) {
      commandConsole.push(`[ERROR] ${String(error)}`, 'error');
    }
  }

  async function handleBaudChange() {
    try {
      await setSerialConfig(selectedPort, selectedBaud);
      commandConsole.push(`[OK] Baudrate seleccionado: ${selectedBaud}`, 'success');
    } catch (error) {
      commandConsole.push(`[ERROR] ${String(error)}`, 'error');
    }
  }
</script>

<section class="panel commands-panel">
  <div class="panel-title">Commands</div>

  <div class="commands-grid">
    {#each commandButtons as item}
      <button
        type="button"
        on:click={() => handleCommand(item.command)}
        disabled={!item.command || $commandConsole.loading}
      >
        {item.label}
      </button>
    {/each}

    <select
      class="command-select"
      bind:value={selectedPort}
      on:change={handlePortChange}
      disabled={$commandConsole.loading}
    >
      {#if availablePorts.length === 0}
        <option value={selectedPort}>No ports</option>
      {:else}
        {#each availablePorts as port}
          <option value={port}>{port}</option>
        {/each}
      {/if}
    </select>

    <select
      class="command-select"
      bind:value={selectedBaud}
      on:change={handleBaudChange}
      disabled={$commandConsole.loading}
    >
      {#each baudOptions as baud}
        <option value={baud}>{baud}</option>
      {/each}
    </select>
  </div>
</section>
