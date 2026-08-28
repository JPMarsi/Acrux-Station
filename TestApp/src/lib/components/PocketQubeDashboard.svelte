<script>
  import HeaderTeam from './HeaderTeam.svelte';
  import PocketQubeTelemetryPanel from './PocketQubeTelemetryPanel.svelte';
  import PocketQubeChartsGrid from './PocketQubeChartsGrid.svelte';
  import RightSidebar from './RightSidebar.svelte';
</script>

<!-- Todo lo contenido aqui pertenece a la vista del PocketQube. -->
<main class="dashboard payload-dashboard" aria-label="Dashboard del PocketQube">
  <HeaderTeam />
  <PocketQubeTelemetryPanel />
  <PocketQubeChartsGrid />
  <RightSidebar />
</main>
