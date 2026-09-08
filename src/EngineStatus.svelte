<script>
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";

  let status = $state({ status: "not_installed" }); // se sobreescribe en onMount
  let loading = $state(true);
  let installing = $state(false);
  let logLines = $state([]);
  let errorMsg = $state("");
  let knownIssuesUrl = $state("");

  const unlisten = [];

  async function refreshStatus() {
    loading = true;
    errorMsg = "";
    try {
      status = await invoke("engine_status");
    } catch (e) {
      errorMsg = String(e);
    } finally {
      loading = false;
    }
  }

  async function install() {
    installing = true;
    logLines = [];
    errorMsg = "";
    try {
      await invoke("engine_install");
    } catch (e) {
      installing = false;
      errorMsg = String(e);
    }
  }

  async function launch() {
    errorMsg = "";
    try {
      await invoke("engine_launch");
    } catch (e) {
      errorMsg = String(e);
    }
  }

  onMount(async () => {
    knownIssuesUrl = await invoke("engine_known_issues_url");
    await refreshStatus();

    unlisten.push(
      await listen("engine://install-log", (event) => {
        logLines = [...logLines, event.payload];
      }),
    );
    unlisten.push(
      await listen("engine://install-done", async () => {
        installing = false;
        await refreshStatus();
      }),
    );
    unlisten.push(
      await listen("engine://install-error", (event) => {
        installing = false;
        errorMsg = event.payload;
      }),
    );
  });

  onDestroy(() => {
    unlisten.forEach((fn) => fn());
  });
</script>

<section class="engine-card">
  <h2>Motor: Cordial</h2>

  {#if loading}
    <p class="muted">Revisando el sistema…</p>
  {:else if status.status === "flatpak_missing"}
    <p class="warn">
      No se encontró <code>flatpak</code> en el sistema. Instalalo desde el
      gestor de paquetes de tu distro y volvé a abrir Vaporstrap.
    </p>
  {:else if status.status === "remote_missing" || status.status === "not_installed"}
    <p class="muted">
      Cordial todavía no está instalado. Vaporstrap va a agregar su remoto de
      Flatpak propio (Cordial no está en Flathub) e instalarlo.
    </p>
    <button onclick={install} disabled={installing}>
      {installing ? "Instalando…" : "Instalar Cordial"}
    </button>
  {:else if status.status === "installed"}
    <p class="ok">Cordial {status.version} instalado.</p>
    <p class="muted small">
      Al abrirlo por primera vez, usá el botón "Download Roblox" dentro de
      Cordial — descarga y verifica el build de Roblox por vos, no hace falta
      que le pases un APK a mano.
    </p>
    <button onclick={launch}>Iniciar Cordial</button>
  {/if}

  {#if installing}
    <pre class="log">{logLines.join("\n")}</pre>
  {/if}

  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}

  {#if knownIssuesUrl}
    <p class="muted small">
      <a href={knownIssuesUrl} target="_blank" rel="noreferrer">
        Ver problemas conocidos de la versión instalada de Cordial ↗
      </a>
    </p>
  {/if}
</section>

<style>
  .engine-card {
    background: #0f1e2e;
    border: 1px solid #1c2c3d;
    border-radius: 12px;
    padding: 1.5rem;
    max-width: 480px;
    width: 100%;
    text-align: left;
    margin-top: 2rem;
  }

  h2 {
    margin-top: 0;
    font-size: 1.1rem;
  }

  .muted {
    color: #8fb4d9;
  }

  .small {
    font-size: 0.8rem;
  }

  .ok {
    color: #7fd99a;
  }

  .warn {
    color: #e0b34d;
  }

  .error {
    color: #e07a7a;
  }

  button {
    background: #1c6fd9;
    color: white;
    border: none;
    border-radius: 8px;
    padding: 0.6rem 1.2rem;
    font-size: 0.9rem;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.6;
    cursor: default;
  }

  .log {
    margin-top: 1rem;
    max-height: 160px;
    overflow-y: auto;
    background: #060d16;
    border-radius: 8px;
    padding: 0.75rem;
    font-size: 0.7rem;
    color: #6b8299;
    white-space: pre-wrap;
    word-break: break-all;
  }
</style>
