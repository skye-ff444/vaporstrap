<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let profiles = $state([]);
  let newProfileName = $state("");
  let joinLink = $state("");
  let statusMsg = $state("");
  let errorMsg = $state("");

  async function refresh() {
    try {
      profiles = await invoke("profiles_list");
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function createProfile() {
    if (!newProfileName.trim()) return;
    errorMsg = "";
    try {
      await invoke("profiles_create", { name: newProfileName.trim() });
      newProfileName = "";
      statusMsg = "Perfil creado. Abrilo desde el selector de perfiles dentro de Cordial.";
      await refresh();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function renameProfile(oldName) {
    const newName = prompt(`Nuevo nombre para "${oldName}":`, oldName);
    if (!newName || newName === oldName) return;
    errorMsg = "";
    try {
      await invoke("profiles_rename", { oldName, newName });
      await refresh();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function deleteProfile(name) {
    if (
      !confirm(
        `¿Borrar el perfil "${name}"? Esto elimina su sesión, sus FastFlags y los datos de Roblox asociados a esa cuenta. No se puede deshacer.`,
      )
    ) {
      return;
    }
    errorMsg = "";
    try {
      await invoke("profiles_delete", { name });
      await refresh();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function launchWithLink() {
    if (!joinLink.trim()) return;
    errorMsg = "";
    statusMsg = "";
    try {
      await invoke("profiles_launch_with_link", { link: joinLink.trim() });
      statusMsg = "Enviado a Cordial.";
    } catch (e) {
      errorMsg = String(e);
    }
  }

  onMount(refresh);
</script>

<section class="profiles-card">
  <h2>Perfiles de Cordial</h2>
  <p class="muted small">
    Cordial es de proceso único: abrir varias cuentas a la vez es un
    selector de perfiles <strong>dentro de la propia interfaz de Cordial</strong>,
    no varios procesos separados (confirmado en su código fuente: es una
    GApplication de ID fijo). Vaporstrap gestiona los perfiles como carpetas
    para que crear una cuenta nueva no signifique tocar
    <code>~/.local/share/cordial</code> a mano — abrilos y cambiá entre ellos
    desde el selector de Cordial.
  </p>

  <div class="row">
    <input type="text" placeholder="Nombre del perfil nuevo" bind:value={newProfileName} />
    <button onclick={createProfile}>Crear</button>
  </div>

  <ul class="profile-list">
    {#each profiles as name}
      <li>
        <code>{name}</code>
        <span class="spacer"></span>
        <button class="link" onclick={() => renameProfile(name)}>Renombrar</button>
        <button class="link danger" onclick={() => deleteProfile(name)}>Borrar</button>
      </li>
    {/each}
    {#if profiles.length === 0}
      <li class="muted small">Ningún perfil todavía — se crean la primera vez que abrís Cordial, o acá arriba.</li>
    {/if}
  </ul>

  <h3>Unirse por link</h3>
  <p class="muted small">
    Envía un link de Roblox (<code>roblox-player:...</code> o <code>roblox:...</code>)
    directo a Cordial. Si ya está abierto, se lo entrega a esa misma ventana.
  </p>
  <div class="row">
    <input type="text" placeholder="roblox-player://..." bind:value={joinLink} />
    <button onclick={launchWithLink}>Enviar</button>
  </div>

  {#if statusMsg}
    <p class="ok">{statusMsg}</p>
  {/if}
  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}
</section>

<style>
  .profiles-card {
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
  }

  input[type="text"] {
    flex: 1;
    background: #060d16;
    color: #e6f0fa;
    border: 1px solid #1c2c3d;
    border-radius: 6px;
    padding: 0.4rem 0.5rem;
    font-size: 0.85rem;
  }

  .profile-list {
    list-style: none;
    padding: 0;
    margin: 0.5rem 0;
  }

  .profile-list li {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0.4rem 0;
    border-bottom: 1px solid #142236;
  }

  .spacer {
    flex: 1;
  }

  code {
    font-family: ui-monospace, monospace;
    font-size: 0.8rem;
    color: #e6f0fa;
  }

  button {
    background: #1c6fd9;
    color: white;
    border: none;
    border-radius: 8px;
    padding: 0.4rem 0.9rem;
    font-size: 0.8rem;
    cursor: pointer;
    white-space: nowrap;
  }

  button.link {
    background: transparent;
    color: #6fb1e0;
    padding: 0.2rem 0.4rem;
  }

  button.link.danger {
    color: #e07a7a;
  }

  .ok {
    color: #7fd99a;
  }

  .error {
    color: #e07a7a;
  }
</style>
