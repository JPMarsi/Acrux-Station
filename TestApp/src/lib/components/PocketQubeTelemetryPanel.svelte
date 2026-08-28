<script>
  import { pocketQubeTelemetry } from '../stores/telemetry';

  // Campos visuales exclusivos del PocketQube.
  $: telemetryRows = Object.entries($pocketQubeTelemetry);
</script>

<section class="panel telemetry-panel">
  <div class="panel-title">Telemetría PocketQube</div>
  <ul class="telemetry-list">
    {#each telemetryRows as [label, value]}
      <li><span>{label.toUpperCase()}</span><strong>= {value}</strong></li>
    {/each}
  </ul>
</section>
