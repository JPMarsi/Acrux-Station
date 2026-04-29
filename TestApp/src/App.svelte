<script>
  const states = ['PRELAUNCH', 'LAUNCH', 'EJECT', 'SP1_R', 'SP2_R', 'LANDED'];

  let activeState = 'PRELAUNCH';

  let telemetry = {
    mode: 'CONTAINER',
    packetCount: 61,
    battery: 3.46,
    altitude: 647.4,
    temperature: 36.6,
    gpsAlt: 0.0,
    time: '19:27:17.14',
    elapsed: '00:01:11.12'
  };

  const charts = [
    { title: 'ALTITUDE', value: 647.4 },
    { title: 'TEMPERATURE', value: 36.6 },
    { title: 'GPS_ALT', value: 0.0 },
    { title: 'PAYLOAD1 ALTITUDE', value: 0 },
    { title: 'PAYLOAD1 TEMPERATURE', value: 0 },
    { title: 'PAYLOAD1 ROTATION', value: 0 },
    { title: 'PAYLOAD2 ALTITUDE', value: 0 },
    { title: 'PAYLOAD2 TEMPERATURE', value: 0 },
    { title: 'PAYLOAD2 ROTATION', value: 0 }
  ];

  const commands = [
    'MQTT', 'CXON', 'SET_TIME', 'SIM_ENABLE', 'SIM_ACTIVATE',
    'MAP', 'CXOFF', 'SIM_DISABLE', 'SEND_PRESSURE'
  ];
</script>

<style> 
:root {
  --bg: #2c2f56;
  --panel: #343863;
  --panel-2: #3d4270;
  --text: #f2f4ff;
  --muted: #b8bfdc;
  --accent: #ff5f70;
  --accent-2: #d36bff;
  --line: #59608f;
}

html, body, #app {
  margin: 0;
  width: 100%;
  height: 100%;
  background: var(--bg);
  color: var(--text);
  font-family: Arial, Helvetica, sans-serif;
}

body {
  min-width: 1200px;
  min-height: 700px;
}

.app {
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.topbar {
  display: flex;
  align-items: center;
  height: 48px;
}

.team {
  font-size: 2rem;
  font-weight: 800;
  letter-spacing: 1px;
}

.state-header h2,
.command-panel h2,
.gis-panel h2 {
  text-align: center;
  color: var(--accent);
  margin: 0 0 8px 0;
}

.progress {
  height: 14px;
  border-radius: 999px;
  background: linear-gradient(90deg, var(--accent-2), #7b8cff);
  margin-bottom: 10px;
}

.states {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 10px;
}

.state-pill {
  text-align: center;
  font-weight: 800;
  padding: 6px 8px;
}

.state-pill.active {
  color: white;
}

.layout {
  display: grid;
  grid-template-columns: 250px 1fr 360px;
  gap: 12px;
}

.left-panel,
.right-panel,
.command-panel,
.gis-panel,
.coords-panel,
.table-panel,
.chart-card,
.payload-card,
.mini-chart {
  background: var(--panel);
  border: 1px solid var(--line);
  border-radius: 8px;
}

.left-panel,
.right-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.left-panel {
  padding: 12px;
}

.info-block {
  margin-bottom: 10px;
}

.label {
  color: var(--muted);
  font-size: 0.9rem;
}

.big-value {
  font-size: 2rem;
  font-weight: 900;
}

.accent {
  color: var(--accent);
}

.info-row {
  display: flex;
  justify-content: space-between;
  padding: 6px 0;
}

.payload-card {
  padding: 12px;
}

.payload-card h3 {
  margin: 0 0 10px 0;
  color: var(--accent);
}

.center-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.mini-charts {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.mini-chart {
  min-height: 90px;
  padding: 10px;
}

.telemetry-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.chart-card {
  padding: 10px;
}

.chart-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-weight: 700;
}

.chart-value {
  color: #7cf0c0;
}

.chart-placeholder {
  height: 120px;
  border: 1px solid var(--line);
  border-radius: 6px;
  background:
    linear-gradient(to top, rgba(255,255,255,0.03), rgba(255,255,255,0.01));
}

.command-panel,
.gis-panel,
.coords-panel,
.table-panel {
  padding: 10px;
}

.command-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.command-grid button {
  border: 1px solid var(--line);
  background: var(--panel-2);
  color: var(--text);
  padding: 8px 6px;
  border-radius: 6px;
  font-weight: 700;
  cursor: pointer;
}

.command-meta {
  margin-top: 10px;
  color: var(--muted);
  font-size: 0.9rem;
}

.gis-box {
  height: 190px;
  background: #ececec;
  border-radius: 4px;
}

.coord-header {
  display: flex;
  justify-content: space-between;
  font-weight: 800;
  margin-bottom: 8px;
}

.table-placeholder {
  height: 130px;
  border: 1px solid var(--line);
}

.footer {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  align-items: center;
  font-weight: 800;
}

.incoming {
  text-align: right;
  color: var(--accent);
}
</style>

<div class="app">
  <header class="topbar">
    <div class="team">ACRUX#3751</div>
  </header>

  <section class="state-header">
    <h2>STATE</h2>
    <div class="progress"></div>
    <div class="states">
      {#each states as state}
        <div class:active={state === activeState} class="state-pill">
          {state}
        </div>
      {/each}
    </div>
  </section>

  <main class="layout">
    <aside class="left-panel">
      <div class="info-block">
        <div class="label">MODE:</div>
        <div class="big-value accent">{telemetry.mode}</div>
      </div>

      <div class="info-row">
        <span>PKG_COUNT</span>
        <span>{telemetry.packetCount}</span>
      </div>

      <div class="info-row">
        <span>BATTERY</span>
        <span>{telemetry.battery}</span>
      </div>

      <div class="payload-card">
        <h3>PAYLOAD_1</h3>
        <div>PKG_COUNT 0</div>
      </div>

      <div class="payload-card">
        <h3>PAYLOAD_2</h3>
        <div>PKG_COUNT 0</div>
      </div>
    </aside>

    <section class="center-panel">
      <div class="mini-charts">
        <div class="mini-chart">PORT / CONNECT / REFRESH</div>
        <div class="mini-chart">Telemetry preview</div>
        <div class="mini-chart">Rotation preview</div>
      </div>

      <div class="telemetry-grid">
        {#each charts as chart}
          <div class="chart-card">
            <div class="chart-header">
              <span>{chart.title}</span>
              <span class="chart-value">{chart.value}</span>
            </div>
            <div class="chart-placeholder"></div>
          </div>
        {/each}
      </div>
    </section>

    <aside class="right-panel">
      <section class="command-panel">
        <h2>COMMAND</h2>
        <div class="command-grid">
          {#each commands as cmd}
            <button>{cmd}</button>
          {/each}
        </div>

        <div class="command-meta">
          <div>LAST CMD: S20N</div>
          <div>FILENAME: sim.txt</div>
        </div>
      </section>

      <section class="gis-panel">
        <h2>GIS</h2>
        <div class="gis-box"></div>
      </section>

      <section class="coords-panel">
        <div class="coord-header">
          <span>LATITUDE</span>
          <span>LONGITUDE</span>
        </div>
        <div>CONTAINER -80- / -80-</div>
        <div>PAYLOAD1 -80- / -80-</div>
        <div>PAYLOAD2 -80- / -80-</div>
      </section>

      <section class="table-panel">
        <div class="table-placeholder"></div>
      </section>
    </aside>
  </main>

  <footer class="footer">
    <div>TIME: {telemetry.time}</div>
    <div>ELAPSED: {telemetry.elapsed}</div>
    <div class="incoming">INCOMING DATA</div>
  </footer>
</div>