<script>
  import { pocketQubeTelemetryHistory } from '../stores/telemetryHistory';
  import ChartCard from './ChartCard.svelte';
</script>

<section class="panel charts-panel pocketqube-charts">
  <div class="chart-grid">
    <ChartCard title="Altitude" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.altitude} color="#005f73" unit=" m" />
    <ChartCard title="Temperature" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.temperature} color="#b91c1c" unit=" °C" />
    <ChartCard title="Voltage" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.voltage} color="#047857" unit=" V" />
    <ChartCard title="Current" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.current} color="#9a3412" unit=" mA" />
    <ChartCard title="Orientation X" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.rot_rate_x} color="#2563eb" unit=" °/s" />
    <ChartCard title="Orientation Y" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.rot_rate_y} color="#7c3aed" unit=" °/s" />
    <ChartCard title="Orientation Z" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.rot_rate_z} color="#db2777" unit=" °/s" />
    <ChartCard title="GNSS Latitude" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.gnss_latitude} color="#0f766e" unit=" °" />
    <ChartCard title="GNSS Longitude" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.gnss_longitude} color="#1d4ed8" unit=" °" />
    <ChartCard title="Solar Panel 1" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.solar_1} color="#a16207" unit=" V" />
    <ChartCard title="Solar Panel 2" labels={$pocketQubeTelemetryHistory.labels} values={$pocketQubeTelemetryHistory.solar_2} color="#ca8a04" unit=" V" />
  </div>
</section>
