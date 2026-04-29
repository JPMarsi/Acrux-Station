<script>
  import { telemetry } from '../stores/telemetry';

  $: telemetryRows = [
    { label: 'TEAM_ID', value: $telemetry.team_id },
    { label: 'MISSION_TIME', value: $telemetry.mission_time },
    { label: 'PACKET_COUNT', value: $telemetry.packet_count },
    { label: 'MODE', value: $telemetry.mode },
    { label: 'STATE', value: $telemetry.state },
    { label: 'ALTITUDE', value: $telemetry.altitude },
    { label: 'TEMPERATURE', value: $telemetry.temperature },
    { label: 'PRESSURE', value: $telemetry.pressure },
    { label: 'VOLTAGE', value: $telemetry.voltage },
    { label: 'CURRENT', value: $telemetry.current },
    { label: 'GYRO_R', value: $telemetry.gyro_r },
    { label: 'GYRO_P', value: $telemetry.gyro_p },
    { label: 'GYRO_Y', value: $telemetry.gyro_y },
    { label: 'ACCEL_R', value: $telemetry.accel_r },
    { label: 'ACCEL_P', value: $telemetry.accel_p },
    { label: 'ACCEL_Y', value: $telemetry.accel_y },
    { label: 'GPS_TIME', value: $telemetry.gps_time },
    { label: 'GPS_ALTITUDE', value: $telemetry.gps_altitude },
    { label: 'GPS_LATITUDE', value: $telemetry.gps_latitude },
    { label: 'GPS_LONGITUDE', value: $telemetry.gps_longitude },
    { label: 'GPS_SATS', value: $telemetry.gps_sats },
    { label: 'CMD_ECHO', value: $telemetry.cmd_echo },
    { label: 'OPTIONAL_DATA', value: $telemetry.optional_data }
  ];
</script>

<section class="panel telemetry-panel">
  <div class="panel-title">Telemetría</div>

  <ul class="telemetry-list">
    {#each telemetryRows as item}
      <li>
        <span>{item.label}</span>
        <strong>= {item.value}</strong>
      </li>
    {/each}
  </ul>
</section>