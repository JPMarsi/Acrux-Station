<script>
  import { telemetryHistory } from '../stores/telemetryHistory';
  import ChartCard from './ChartCard.svelte';
</script>

<section class="panel charts-panel">
  <div class="chart-grid">
    <ChartCard
      title="Altitude"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.altitude}
      color="#0f766e"
      unit=" m"
    />

    <ChartCard
      title="Temperature"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.temperature}
      color="#c2410c"
      unit=" °C"
    />

    <ChartCard
      title="Pressure"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.pressure}
      color="#7c3aed"
      unit=" hPa"
    />

    <ChartCard
      title="Voltage"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.voltage}
      color="#15803d"
      unit=" V"
    />

    <ChartCard
      title="Current"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.current}
      color="#b45309"
      unit=" A"
    />

    <ChartCard
      title="GPS Altitude"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.gps_altitude}
      color="#1f2937"
      unit=" m"
    />
  </div>
</section>