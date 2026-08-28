<script>
  import HeaderTeam from './HeaderTeam.svelte';
  import ContainerTelemetryPanel from './ContainerTelemetryPanel.svelte';
  import ContainerChartsGrid from './ContainerChartsGrid.svelte';
  import RightSidebar from './RightSidebar.svelte';
</script>

<!-- Todo lo contenido aqui pertenece a la vista del Container. -->
<main class="dashboard payload-dashboard" aria-label="Dashboard del Container">
  <HeaderTeam />
  <ContainerTelemetryPanel />
  <ContainerChartsGrid />
  <RightSidebar />
</main>
