import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// Config recomendada por Tauri: https://v2.tauri.app/start/frontend/
export default defineConfig(async () => ({
  plugins: [svelte()],

  // Evita que Vite oculte errores de Rust en la consola
  clearScreen: false,

  // Tauri necesita un puerto fijo y predecible
  server: {
    port: 1420,
    strictPort: true,
    host: process.env.TAURI_DEV_HOST || false,
    watch: {
      // Ignora cambios en src-tauri para evitar rebuilds cruzados
      ignored: ["**/src-tauri/**"],
    },
  },

  // Variables de entorno para distintos targets de Tauri
  envPrefix: ["VITE_", "TAURI_ENV_*"],

  build: {
    target: process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
}));
