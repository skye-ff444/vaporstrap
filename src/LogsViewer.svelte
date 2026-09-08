<script>
  import { invoke } from "@tauri-apps/api/core";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import { onMount } from "svelte";

  let vaporstrapLog = $state("");
  let launches = $state([]);
  let selectedLaunch = $state("");
  let launchLog = $state("");
  let logsDir = $state("");
  let statusMsg = $state("");
  let errorMsg = $state("");

  async function refreshVaporstrapLog() {
    try {
      vaporstrapLog = await invoke("logs_read_vaporstrap");
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function refreshLaunches() {
    try {
      launches = await invoke("logs_list_cordial_launches");
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function openLaunchLog(name) {
    selectedLaunch = name;
    errorMsg = "";
    try {
      launchLog = await invoke("logs_read", { name });
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function openLogsFolder() {
    errorMsg = "";
    try {
      if (!logsDir) {
        logsDir = await invoke("logs_dir_path");
      }
      await revealItemInDir(logsDir);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function generateReport() {
    errorMsg = "";
    statusMsg = "";
    try {
      const path = await invoke("logs_generate_diagnostic_report");
      statusMsg = `Reporte guardado en: ${path}`;
      await refreshLaunches();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  function formatDate(unixSecs) {
    return new Date(unixSecs * 1000).toLocaleString();
  }

  onMount(async () => {
    await refreshVaporstrapLog();
    await refreshLaunches();
  });
</script>

<section class="logs-card">
  <h2>Logs y diagnóstico</h2>
  <p class="muted small">
    Todo local — nada de esto se envía a ningún lado automáticamente. El
    reporte de diagnóstico junta specs de hardware, estado del motor y las
    últimas líneas de los logs en un archivo de texto para que vos decidas
    qué hacer con él (adjuntarlo a un issue, mandarlo por Discord, etc).
  </p>

  <div class="row">
    <button onclick={generateReport}>Generar reporte de diagnóstico</button>
    <button onclick={openLogsFolder}>Abrir carpeta de logs</button>
  </div>

  <h3>Log de Vaporstrap</h3>
  <pre class="log">{vaporstrapLog || "(vacío todavía)"}</pre>

  <h3>Lanzamientos de Cordial</h3>
  {#if launches.length === 0}
    <p class="muted small">Todavía no se lanzó Cordial desde Vaporstrap.</p>
  {/if}
  <ul class="launch-list">
    {#each launches as launch}
      <li>
        <button class="link" onclick={() => openLaunchLog(launch.name)}>
          {launch.name}
        </button>
        <span class="muted small">
          {formatDate(launch.modified_unix)} · {(launch.size_bytes / 1024).toFixed(1)} KB
        </span>
      </li>
    {/each}
  </ul>

  {#if selectedLaunch}
    <h3>Log: {selectedLaunch}</h3>
    <pre class="log">{launchLog || "(vacío)"}</pre>
  {/if}

  {#if statusMsg}
    <p class="ok">{statusMsg}</p>
  {/if}
  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}
</section>

<style>
  .logs-card {
    background: #0f1e2e;
    border: 1px solid #1c2c3d;
    border-radius: 12px;
    padding: 1.5rem;
    max-width: 620px;
    width: 100%;
    text-align: left;
    margin-top: 1.5rem;
  }

  h2 {
    margin-top: 0;
    font-size: 1.1rem;
  }

  h3 {
    font-size: 0.95rem;
    color: #cfe3f7;
    margin: 1rem 0 0.4rem;
  }

  .muted {
    color: #8fb4d9;
  }

  .small {
    font-size: 0.8rem;
  }

  .row {
    display: flex;
    gap: 0.5rem;
    margin: 0.6rem 0;
    flex-wrap: wrap;
  }

  .log {
    max-height: 220px;
    overflow-y: auto;
    background: #060d16;
    border-radius: 8px;
    padding: 0.75rem;
    font-size: 0.72rem;
    color: #8fb4d9;
    white-space: pre-wrap;
    word-break: break-all;
  }

  .launch-list {
    list-style: none;
    padding: 0;
    margin: 0.3rem 0;
  }

  .launch-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    padding: 0.3rem 0;
    border-bottom: 1px solid #142236;
  }

  button {
    background: #1c6fd9;
    color: white;
    border: none;
    border-radius: 8px;
    padding: 0.5rem 1rem;
    font-size: 0.85rem;
    cursor: pointer;
  }

  button.link {
    background: transparent;
    color: #6fb1e0;
    padding: 0;
    font-size: 0.8rem;
    text-align: left;
  }

  .ok {
    color: #7fd99a;
  }

  .error {
    color: #e07a7a;
  }
</style>
