<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let snapshot = $state(null);
  let recommendation = $state(null);
  let probing = $state(false);
  let cordialProfiles = $state([]);
  let selectedProfile = $state("default");
  let currentEnv = $state({ performance: null, graphics: null });
  let statusMsg = $state("");
  let errorMsg = $state("");

  async function refreshProfiles() {
    try {
      cordialProfiles = await invoke("fflags_list_cordial_profiles");
      if (cordialProfiles.length > 0 && !cordialProfiles.includes(selectedProfile)) {
        selectedProfile = cordialProfiles[0];
      }
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function refreshCurrentEnv() {
    try {
      currentEnv = await invoke("hardware_current_launch_env");
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function analyze() {
    probing = true;
    errorMsg = "";
    statusMsg = "";
    recommendation = null;
    try {
      snapshot = await invoke("hardware_probe");
      recommendation = await invoke("hardware_recommend", { snapshot });
    } catch (e) {
      errorMsg = String(e);
    } finally {
      probing = false;
    }
  }

  async function apply() {
    if (!recommendation) return;
    errorMsg = "";
    try {
      await invoke("hardware_apply", { profile: selectedProfile, recommendation });
      statusMsg = `Aplicado: se guardó para el próximo lanzamiento y se actualizó el flags.json del perfil "${selectedProfile}".`;
      await refreshCurrentEnv();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  onMount(async () => {
    await refreshProfiles();
    await refreshCurrentEnv();
  });
</script>

<section class="hw-card">
  <h2>Optimizar mi laptop</h2>
  <p class="muted small">
    Sondea CPU/RAM/Vulkan y recomienda uno de los modos nativos de Cordial
    (<code>CORDIAL_PERFORMANCE</code> / <code>CORDIAL_GRAPHICS</code>) más un
    FPS cap y nivel de calidad gráfica. Es una heurística simple, no un
    benchmark certificado — revisá el razonamiento antes de aplicar.
  </p>

  {#if currentEnv.performance || currentEnv.graphics}
    <p class="muted small">
      Optimización guardada actualmente: rendimiento
      <strong>{currentEnv.performance ?? "balanced (default)"}</strong>,
      gráficos <strong>{currentEnv.graphics ?? "automatic (default)"}</strong>.
    </p>
  {/if}

  <button onclick={analyze} disabled={probing}>
    {probing ? "Analizando…" : "Analizar mi hardware"}
  </button>

  {#if snapshot}
    <div class="snapshot">
      <p><strong>Núcleos físicos:</strong> {snapshot.physical_cores} ({snapshot.logical_cores} hilos)</p>
      <p><strong>RAM:</strong> {snapshot.ram_mb ? `${(snapshot.ram_mb / 1024).toFixed(1)} GB` : "no detectada"}</p>
      <p><strong>Vulkan (ICD instalado):</strong> {snapshot.vulkan_icd_found ? "sí" : "no encontrado"}</p>
      {#if snapshot.gpu_hint}
        <p><strong>GPU (lspci):</strong> {snapshot.gpu_hint}</p>
      {/if}
      <p><strong>Microbenchmark CPU:</strong> {snapshot.cpu_bench_ms.toFixed(0)} ms</p>
    </div>
  {/if}

  {#if recommendation}
    <div class="recommendation">
      <h3>Recomendación</h3>
      <ul>
        <li>Rendimiento: <code>{recommendation.performance}</code></li>
        <li>Gráficos: <code>{recommendation.graphics}</code></li>
        <li>FPS cap: <code>{recommendation.fps_cap}</code></li>
        <li>Calidad gráfica: <code>{recommendation.graphics_quality}/10</code></li>
      </ul>
      <p class="muted small">Por qué:</p>
      <ul class="reasoning">
        {#each recommendation.reasoning as line}
          <li>{line}</li>
        {/each}
      </ul>

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
        <button onclick={apply}>Aplicar</button>
      </div>
    </div>
  {/if}

  {#if statusMsg}
    <p class="ok">{statusMsg}</p>
  {/if}
  {#if errorMsg}
    <p class="error">{errorMsg}</p>
  {/if}
</section>

<style>
  .hw-card {
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
    margin: 0.8rem 0 0.4rem;
  }

  .muted {
    color: #8fb4d9;
  }

  .small {
    font-size: 0.8rem;
  }

  .snapshot,
  .recommendation {
    margin-top: 1rem;
    padding-top: 0.75rem;
    border-top: 1px solid #142236;
    font-size: 0.85rem;
  }

  .snapshot p {
    margin: 0.2rem 0;
  }

  ul {
    margin: 0.3rem 0;
    padding-left: 1.2rem;
  }

  .reasoning li {
    color: #8fb4d9;
    font-size: 0.78rem;
    margin-bottom: 0.2rem;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin-top: 0.8rem;
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
    padding: 0.5rem 1rem;
    font-size: 0.85rem;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.6;
    cursor: default;
  }

  select {
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
