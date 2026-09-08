<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
  import { onMount } from "svelte";

  let catalog = $state([]);
  let search = $state("");
  let entries = $state([]); // FlagEntry[] del preset actual
  let presetName = $state("sin guardar");
  let presetList = $state([]);
  let cordialProfiles = $state([]);
  let selectedProfile = $state("default");
  let statusMsg = $state("");
  let errorMsg = $state("");

  let filteredCatalog = $derived(
    search.trim().length === 0
      ? catalog
      : catalog.filter(
          (c) =>
            c.name.toLowerCase().includes(search.toLowerCase()) ||
            c.description.toLowerCase().includes(search.toLowerCase()),
        ),
  );

  function isInPreset(name) {
    return entries.some((e) => e.name === name);
  }

  function addFromCatalog(catalogEntry) {
    if (isInPreset(catalogEntry.name)) return;
    entries = [
      ...entries,
      {
        name: catalogEntry.name,
        enabled: true,
        value: catalogEntry.default_value,
        description: catalogEntry.description,
      },
    ];
  }

  function addCustomFlag() {
    const name = prompt("Nombre exacto de la flag (p. ej. FFlagAlgo o DFIntOtraCosa):");
    if (!name || !name.trim()) return;
    entries = [
      ...entries,
      {
        name: name.trim(),
        enabled: true,
        value: { type: "bool", value: true },
        description: null,
      },
    ];
  }

  function removeEntry(name) {
    entries = entries.filter((e) => e.name !== name);
  }

  function toggleEntry(name) {
    entries = entries.map((e) => (e.name === name ? { ...e, enabled: !e.enabled } : e));
  }

  function updateValue(name, rawInput) {
    entries = entries.map((e) => {
      if (e.name !== name) return e;
      if (e.value.type === "bool") {
        return { ...e, value: { type: "bool", value: rawInput } };
      }
      if (e.value.type === "int") {
        const n = parseInt(rawInput, 10);
        return { ...e, value: { type: "int", value: Number.isNaN(n) ? 0 : n } };
      }
      return { ...e, value: { type: "str", value: rawInput } };
    });
  }

  async function refreshPresetList() {
    try {
      presetList = await invoke("fflags_list_presets");
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function refreshCordialProfiles() {
    try {
      cordialProfiles = await invoke("fflags_list_cordial_profiles");
      if (cordialProfiles.length > 0 && !cordialProfiles.includes(selectedProfile)) {
        selectedProfile = cordialProfiles[0];
      }
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function savePreset() {
    const name = prompt("Nombre del preset:", presetName === "sin guardar" ? "" : presetName);
    if (!name || !name.trim()) return;
    presetName = name.trim();
    errorMsg = "";
    try {
      await invoke("fflags_save_preset", { preset: { name: presetName, flags: entries } });
      statusMsg = `Preset "${presetName}" guardado.`;
      await refreshPresetList();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function loadPreset(name) {
    errorMsg = "";
    try {
      const preset = await invoke("fflags_load_preset", { name });
      presetName = preset.name;
      entries = preset.flags;
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function deletePreset(name) {
    if (!confirm(`¿Borrar el preset "${name}"?`)) return;
    try {
      await invoke("fflags_delete_preset", { name });
      if (presetName === name) {
        presetName = "sin guardar";
        entries = [];
      }
      await refreshPresetList();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function importJson() {
    errorMsg = "";
    try {
      const filePath = await open({
        multiple: false,
        filters: [{ name: "JSON", extensions: ["json"] }],
      });
      if (!filePath) return;
      const text = await readTextFile(filePath);
      const imported = await invoke("fflags_import_json", { text });
      entries = imported;
      statusMsg = `Se importaron ${imported.length} flags.`;
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function exportJson() {
    errorMsg = "";
    try {
      const json = await invoke("fflags_export_json", { entries });
      const filePath = await save({
        defaultPath: `${presetName === "sin guardar" ? "flags" : presetName}.json`,
        filters: [{ name: "JSON", extensions: ["json"] }],
      });
      if (!filePath) return;
      await writeTextFile(filePath, json);
      statusMsg = "Exportado.";
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function applyToCordial() {
    if (
      !confirm(
        `Esto va a reemplazar por completo el archivo flags.json del perfil "${selectedProfile}" de Cordial con las ${
          entries.filter((e) => e.enabled).length
        } flags habilitadas de este preset. ¿Continuar?`,
      )
    ) {
      return;
    }
    errorMsg = "";
    try {
      await invoke("fflags_apply_to_cordial", { profile: selectedProfile, entries });
      statusMsg = `Aplicado al perfil "${selectedProfile}" de Cordial.`;
    } catch (e) {
      errorMsg = String(e);
    }
  }

  onMount(async () => {
    try {
      catalog = await invoke("fflags_catalog");
    } catch (e) {
      errorMsg = String(e);
    }
    await refreshPresetList();
    await refreshCordialProfiles();
  });
</script>

<section class="fflags-card">
  <h2>Editor de FastFlags</h2>
  <p class="muted small">
    Preset actual: <strong>{presetName}</strong> ({entries.filter((e) => e.enabled).length} habilitadas
    de {entries.length})
  </p>

  <div class="row">
    <select onchange={(e) => e.target.value && loadPreset(e.target.value)}>
      <option value="">Cargar preset…</option>
      {#each presetList as name}
        <option value={name}>{name}</option>
      {/each}
    </select>
    <button onclick={savePreset}>Guardar preset</button>
    {#if presetName !== "sin guardar"}
      <button class="danger" onclick={() => deletePreset(presetName)}>Borrar</button>
    {/if}
  </div>

  <div class="row">
    <button onclick={importJson}>Importar JSON</button>
    <button onclick={exportJson}>Exportar JSON</button>
    <button onclick={addCustomFlag}>+ Flag personalizada</button>
  </div>

  <h3>Flags en el preset</h3>
  {#if entries.length === 0}
    <p class="muted small">Ningún flag todavía. Buscá en el catálogo abajo o agregá una personalizada.</p>
  {/if}
  <ul class="entry-list">
    {#each entries as entry (entry.name)}
      <li>
        <input
          type="checkbox"
          checked={entry.enabled}
          onchange={() => toggleEntry(entry.name)}
        />
        <code>{entry.name}</code>
        {#if entry.value.type === "bool"}
          <select
            value={String(entry.value.value)}
            onchange={(e) => updateValue(entry.name, e.target.value === "true")}
          >
            <option value="true">true</option>
            <option value="false">false</option>
          </select>
        {:else}
          <input
            type={entry.value.type === "int" ? "number" : "text"}
            value={entry.value.value}
            oninput={(e) => updateValue(entry.name, e.target.value)}
          />
        {/if}
        <button class="link" onclick={() => removeEntry(entry.name)}>Quitar</button>
      </li>
    {/each}
  </ul>

  <h3>Catálogo</h3>
  <input class="search" type="text" placeholder="Buscar flag…" bind:value={search} />
  <ul class="catalog-list">
    {#each filteredCatalog as c}
      <li>
        <div>
          <code>{c.name}</code>
          <span class="badge">{c.category}</span>
          <p class="muted small">{c.description}</p>
        </div>
        <button disabled={isInPreset(c.name)} onclick={() => addFromCatalog(c)}>
          {isInPreset(c.name) ? "Agregado" : "+ Agregar"}
        </button>
      </li>
    {/each}
  </ul>
  <p class="muted small">
    Catálogo de referencia, no verificado contra la build actual de Roblox — puede haber
    nombres desactualizados. Usá "Flag personalizada" o importá un JSON para lo que falte.
  </p>

  <h3>Aplicar</h3>
  <div class="row">
    <label>
      Perfil de Cordial:
      <select bind:value={selectedProfile}>
        {#if cordialProfiles.length === 0}
          <option value="default">default</option>
        {/if}
        {#each cordialProfiles as p}
          <option value={p}>{p}</option>
        {/each}
      </select>
    </label>
    <button onclick={applyToCordial}>Aplicar a Cordial</button>
  </div>

  {#if statusMsg}
    <p class="ok">{statusMsg}</p>
  {/if}
  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}
</section>

<style>
  .fflags-card {
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
    margin-bottom: 0.4rem;
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
    align-items: center;
    margin: 0.6rem 0;
    flex-wrap: wrap;
  }

  .search {
    width: 100%;
    padding: 0.5rem;
    border-radius: 6px;
    border: 1px solid #1c2c3d;
    background: #060d16;
    color: #e6f0fa;
    margin-bottom: 0.5rem;
  }

  ul {
    list-style: none;
    padding: 0;
    margin: 0 0 1rem 0;
  }

  .entry-list li {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.35rem 0;
    border-bottom: 1px solid #142236;
  }

  .entry-list code {
    flex: 1;
  }

  .catalog-list {
    max-height: 260px;
    overflow-y: auto;
  }

  .catalog-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0;
    border-bottom: 1px solid #142236;
  }

  .badge {
    font-size: 0.65rem;
    background: #142236;
    color: #8fb4d9;
    padding: 0.1rem 0.4rem;
    border-radius: 999px;
    margin-left: 0.4rem;
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

  button:disabled {
    opacity: 0.5;
    cursor: default;
  }

  button.danger {
    background: #b03a3a;
  }

  button.link {
    background: transparent;
    color: #e07a7a;
    padding: 0;
  }

  select,
  input[type="text"],
  input[type="number"] {
    background: #060d16;
    color: #e6f0fa;
    border: 1px solid #1c2c3d;
    border-radius: 6px;
    padding: 0.3rem 0.5rem;
    font-size: 0.8rem;
  }

  .ok {
    color: #7fd99a;
  }

  .error {
    color: #e07a7a;
  }
</style>
