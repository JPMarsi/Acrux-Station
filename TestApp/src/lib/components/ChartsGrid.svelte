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
      color="#005f73"
      unit=" m"
    />

    <ChartCard
      title="Temperature"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.temperature}
      color="#b91c1c"
      unit=" °C"
    />

    <ChartCard
      title="Pressure"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.pressure}
      color="#6d28d9"
      unit=" hPa"
    />

    <ChartCard
      title="Voltage"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.voltage}
      color="#047857"
      unit=" V"
    />

    <ChartCard
      title="Current"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.current}
      color="#9a3412"
      unit=" A"
    />

    <ChartCard
      title="GPS Altitude"
      labels={$telemetryHistory.labels}
      values={$telemetryHistory.gps_altitude}
      color="#111827"
      unit=" m"
    />
  </div>
</section>
