<script lang="ts">
  import { onMount } from 'svelte';
  import { executeProtocolCommand } from '../services/commandExecutor';
  import { commandConsole } from '../stores/commandConsole';
  import { csvRecording } from '../stores/csvRecording';
  import { linkStatus } from '../stores/linkStatus';
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
  let espnowPort = 'COM6';
  let espnowBaud = 115200;
  let xbeePort = 'COM7';
  let xbeeBaud = 115200;

  onMount(async () => {
    try {
      availablePorts = await listSerialPorts();

      const config = await getSerialConfig();
      espnowPort = config.espnow_port;
      espnowBaud = config.espnow_baud_rate;
      xbeePort = config.xbee_port;
      xbeeBaud = config.xbee_baud_rate;
    } catch (error) {
      console.error('Error loading serial config:', error);
    }
  });
  async function refreshPorts() {
  try {
    availablePorts = await listSerialPorts();

    commandConsole.push(
      `[OK] ${availablePorts.length} puertos encontrados`,
      'success'
    );
  } catch (error) {
    commandConsole.push(
      `[ERROR] ${String(error)}`,
      'error'
    );
  }
}

  async function handleCommand(command: string) {
    if (!command) return;
    await executeProtocolCommand(command);
  }

  async function handleCsvToggle() {
    if ($csvRecording.active) {
      await executeProtocolCommand('CSV STOP');
      return;
    }

    await executeProtocolCommand('CSV START');
  }

  async function saveSerialConfig() {
    try {
      await setSerialConfig({
        espnow_port: espnowPort,
        espnow_baud_rate: espnowBaud,
        xbee_port: xbeePort,
        xbee_baud_rate: xbeeBaud
      });
      commandConsole.push(`[OK] Enlaces configurados: ESP-NOW ${espnowPort} | XBEE ${xbeePort}`, 'success');
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

    <button
      type="button"
      class="csv-toggle"
      class:recording={$csvRecording.active}
      on:click={handleCsvToggle}
      disabled={$commandConsole.loading}
      title={$csvRecording.active ? 'Detener grabación CSV' : 'Iniciar grabación CSV'}
    >
      CSV
    </button>

    <div class="link-config">
      <label
        for="espnow-port"
        class:active={$linkStatus.container_source === 'ESP-NOW' || $linkStatus.pocketqube_source === 'ESP-NOW'}
        title={$linkStatus.espnow_connected ? 'ESP-NOW conectado' : 'ESP-NOW desconectado'}
      >ESP-NOW</label>
      <select id="espnow-port" class="command-select" bind:value={espnowPort} on:click={refreshPorts} on:change={saveSerialConfig} disabled={$commandConsole.loading}>
        {#if availablePorts.length === 0}<option value={espnowPort}>No ports</option>{:else}{#each availablePorts as port}<option value={port}>{port}</option>{/each}{/if}
      </select>
      <select class="command-select" aria-label="Baudrate ESP-NOW" bind:value={espnowBaud} on:change={saveSerialConfig} disabled={$commandConsole.loading}>
        {#each baudOptions as baud}<option value={baud}>{baud}</option>{/each}
      </select>
    </div>

    <div class="link-config">
      <label
        for="xbee-port"
        class:active={$linkStatus.container_source === 'XBEE' || $linkStatus.pocketqube_source === 'XBEE'}
        title={$linkStatus.xbee_connected ? 'XBEE conectado' : 'XBEE desconectado'}
      >XBEE</label>
      <select id="xbee-port" class="command-select" bind:value={xbeePort} on:click={refreshPorts} on:change={saveSerialConfig} disabled={$commandConsole.loading}>
        {#if availablePorts.length === 0}<option value={xbeePort}>No ports</option>{:else}{#each availablePorts as port}<option value={port}>{port}</option>{/each}{/if}
      </select>
      <select class="command-select" aria-label="Baudrate XBEE" bind:value={xbeeBaud} on:change={saveSerialConfig} disabled={$commandConsole.loading}>
        {#each baudOptions as baud}<option value={baud}>{baud}</option>{/each}
      </select>
    </div>
  </div>
</section>
