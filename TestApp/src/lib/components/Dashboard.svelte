<script>
  import '../styles/dashboard.css';
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  import ContainerDashboard from './ContainerDashboard.svelte';
  import PocketQubeDashboard from './PocketQubeDashboard.svelte';

  import {
    initTelemetry,
    destroyTelemetryListener
  } from '../services/telemetry';

  let payloadView = 'container';

  onMount(async () => {
    try {
      payloadView = getCurrentWindow().label === 'pocketqube' ? 'pocketqube' : 'container';
    } catch {
      // En un navegador normal se muestra Container para poder revisar el frontend.
      payloadView = 'container';
    }
    await initTelemetry();
  });

  onDestroy(() => {
    destroyTelemetryListener();
  });
</script>

{#if payloadView === 'pocketqube'}
  <PocketQubeDashboard />
{:else}
  <ContainerDashboard />
{/if}
