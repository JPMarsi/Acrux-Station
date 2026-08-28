<script>
  import { containerTelemetryHistory } from '../stores/telemetryHistory';
  import ChartCard from './ChartCard.svelte';
</script>

<section class="panel charts-panel">
  <div class="chart-grid">
    <ChartCard title="Altitude" labels={$containerTelemetryHistory.labels} values={$containerTelemetryHistory.altitude} color="#005f73" unit=" m" />
    <ChartCard title="Temperature" labels={$containerTelemetryHistory.labels} values={$containerTelemetryHistory.temperature} color="#b91c1c" unit=" °C" />
    <ChartCard title="Battery Current" labels={$containerTelemetryHistory.labels} values={$containerTelemetryHistory.batt_i} color="#9a3412" unit=" mA" />
  </div>
</section>
