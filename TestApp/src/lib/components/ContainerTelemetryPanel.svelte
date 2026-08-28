<script>
  import { containerTelemetry } from '../stores/telemetry';

  // Campos visuales exclusivos del Container.
  $: telemetryRows = Object.entries($containerTelemetry);
</script>

<section class="panel telemetry-panel">
  <div class="panel-title">Telemetría Container</div>
  <ul class="telemetry-list">
    {#each telemetryRows as [label, value]}
      <li><span>{label.toUpperCase()}</span><strong>= {value}</strong></li>
    {/each}
  </ul>
</section>
