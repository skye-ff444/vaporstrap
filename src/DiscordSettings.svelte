<script>
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { onMount } from "svelte";

  let enabled = $state(false);
  let clientId = $state("");
  let statusMsg = $state("");
  let errorMsg = $state("");

  async function load() {
    try {
      const settings = await invoke("discord_get_settings");
      enabled = settings.enabled;
      clientId = settings.client_id ?? "";
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function save() {
    errorMsg = "";
    try {
      await invoke("discord_save_settings", {
        newSettings: { enabled, client_id: clientId.trim() || null },
      });
      statusMsg = "Configuración guardada.";
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function test() {
    errorMsg = "";
    statusMsg = "";
    try {
      await save();
      await invoke("discord_test");
      statusMsg = "Actividad de prueba enviada — revisá tu perfil de Discord.";
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function clear() {
    errorMsg = "";
    try {
      await invoke("discord_clear");
      statusMsg = "Actividad limpiada.";
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function openDeveloperPortal() {
    try {
      await openUrl("https://discord.com/developers/applications");
    } catch (e) {
      errorMsg = String(e);
    }
  }

  onMount(load);
</script>

<section class="discord-card">
  <h2>Discord Rich Presence</h2>
  <p class="muted small">
    Cordial trae su propio plugin de presencia, pero el host de plugins del
    cliente todavía no implementa la suscripción a eventos que ese plugin
    necesita (confirmado en su código fuente). Por eso esto es una
    implementación aparte, propia de Vaporstrap: solo puede mostrar "hay una
    sesión de Cordial corriendo, lanzada desde Vaporstrap" con la hora de
    inicio — no a qué experiencia estás jugando, esa info no está disponible
    todavía por ningún canal externo.
  </p>

  <label class="row">
    <input type="checkbox" bind:checked={enabled} />
    Habilitar Discord Rich Presence
  </label>

  <label class="field">
    Client ID de Discord
    <input
      type="text"
      placeholder="Pegá acá el Client ID de tu aplicación"
      bind:value={clientId}
    />
  </label>
  <p class="muted small">
    Creá una aplicación gratis en
    <button class="link" onclick={openDeveloperPortal}>discord.com/developers/applications ↗</button>
    y copiá su "Application ID". Opcionalmente, subí una imagen con la clave
    <code>vaporstrap-logo</code> en la sección "Rich Presence Assets" de esa
    misma aplicación para que aparezca como ícono grande.
  </p>

  <div class="row">
    <button onclick={save}>Guardar</button>
    <button onclick={test}>Probar ahora</button>
    <button class="danger" onclick={clear}>Limpiar actividad</button>
  </div>

  {#if statusMsg}
    <p class="ok">{statusMsg}</p>
  {/if}
  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}
</section>

<style>
  .discord-card {
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

  .muted {
    color: #8fb4d9;
  }

  .small {
    font-size: 0.8rem;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin: 0.6rem 0;
    flex-wrap: wrap;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    font-size: 0.85rem;
    color: #cfe3f7;
    margin: 0.6rem 0;
  }

  input[type="text"] {
    background: #060d16;
    color: #e6f0fa;
    border: 1px solid #1c2c3d;
    border-radius: 6px;
    padding: 0.4rem 0.5rem;
    font-size: 0.85rem;
  }

  code {
    font-family: ui-monospace, monospace;
    font-size: 0.8rem;
    color: #e6f0fa;
  }

  button.link {
    background: transparent;
    color: #6fb1e0;
    padding: 0;
    font-size: inherit;
    text-decoration: underline;
    display: inline;
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

  button.danger {
    background: #b03a3a;
  }

  .ok {
    color: #7fd99a;
  }

  .error {
    color: #e07a7a;
  }
</style>
