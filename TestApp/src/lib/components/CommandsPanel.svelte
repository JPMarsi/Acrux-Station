<script>
  const commandButtons = [
    'SET TIME',
    'START TELEMETRY',
    'END TELEMETRY',
    'CAL ALTITUDE',
    'CAL PYR',
    'ENABLE SIM',
    'ACTIVATE SIM',
    'DEACTIVATE SIM',
    'OPTION 1',
    'OPTION 2'
  ];
</script>

<section class="panel commands-panel">
  <div class="panel-title">Commands</div>

  <div class="commands-grid">
    {#each commandButtons as command}
      <button type="button">{command}</button>
    {/each}
  </div>
</section>