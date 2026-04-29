<script>
  import '../styles/dashboard.css';
  import { onMount, onDestroy } from 'svelte';

  import HeaderTeam from './HeaderTeam.svelte';
  import AlertStrip from './AlertStrip.svelte';
  
  import TelemetryPanel from './TelemetryPanel.svelte';
  import ChartsGrid from './ChartsGrid.svelte';
  import RightSidebar from './RightSidebar.svelte';

  import {
    initTelemetry,
    destroyTelemetryListener
  } from '../services/telemetry';

  onMount(() => {
    initTelemetry();
  });

  onDestroy(() => {
    destroyTelemetryListener();
  });
</script>

<main class="dashboard">
  <HeaderTeam />
  <AlertStrip />

  <TelemetryPanel />
  <ChartsGrid />
  <RightSidebar />
</main>