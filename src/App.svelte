<script>
  import { getVersion } from "@tauri-apps/api/app";
  import EngineStatus from "./EngineStatus.svelte";
  import FFlagsEditor from "./FFlagsEditor.svelte";
  import HardwareOptimizer from "./HardwareOptimizer.svelte";
  import ProfilesManager from "./ProfilesManager.svelte";
  import DiscordSettings from "./DiscordSettings.svelte";
  import LogsViewer from "./LogsViewer.svelte";

  let version = $state("0.1.0");

  getVersion()
    .then((v) => (version = v))
    .catch(() => {
      // Fuera de un contexto Tauri (p. ej. `npm run dev` en el navegador)
    });
</script>

<main>
  <h1>Vaporstrap</h1>
  <p class="tagline">Bootstrapper de Roblox para Linux</p>
  <p class="version">v{version}</p>

  <EngineStatus />
  <FFlagsEditor />
  <HardwareOptimizer />
  <ProfilesManager />
  <DiscordSettings />
  <LogsViewer />

  <div class="disclaimer">
    Proyecto no oficial, no afiliado a Roblox Corporation. Roblox® es marca
    registrada de Roblox Corporation. Úsalo bajo tu propio riesgo.
  </div>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    background: #0a1420;
    color: #e6f0fa;
    font-family: system-ui, sans-serif;
    text-align: center;
    padding: 2rem;
    box-sizing: border-box;
  }

  h1 {
    font-size: 2.5rem;
    margin-bottom: 0.25rem;
  }

  .tagline {
    color: #8fb4d9;
    margin-top: 0;
  }

  .version {
    font-size: 0.85rem;
    color: #5c7a99;
  }

  .disclaimer {
    margin-top: 3rem;
    max-width: 480px;
    font-size: 0.75rem;
    color: #6b8299;
    border-top: 1px solid #1c2c3d;
    padding-top: 1rem;
  }
</style>
