<script>
  const terminalLines = [
    '[INFO] Sistema inicializado',
    '[INFO] Esperando telemetría...',
    '[WARN] Sin eventos críticos'
  ];
</script>

<section class="panel terminal-panel">
  <div class="panel-title">Terminal</div>

  <div class="terminal-window">
    {#each terminalLines as line}
      <p>{line}</p>
    {/each}
  </div>
</section>