# Arquitectura de Vaporstrap

## Visión general

Vaporstrap se divide en dos capas claramente separadas:

1. **GUI/orquestador (este repo):** Rust + Tauri v2 de backend, Svelte de
   frontend. Se encarga de configuración, UX y de invocar al motor.
2. **Motor (externo, intercambiable):** actualmente
   [Cordial](https://github.com/luohoa97/cordial) (GPL-3.0), que carga
   `libroblox.so` nativamente en Linux (linker bionic + shim JNI + capa de
   framework), sin Wine, sin emulador y sin VM. Si en el futuro
   [Sober](https://sober.vinegarhq.org/) vuelve a ser open source, se podrá
   soportar como backend alternativo.

```
┌─────────────────────────────────────────────┐
│                 VAPORSTRAP (GUI)             │
│  Tauri (Rust) + Svelte                       │
│  - Editor de FastFlags                       │
│  - Perfiles de optimización por hardware     │
│  - Gestor de mods / temas / canales          │
│  - Multi-instancia, Discord RPC, logs        │
└───────────────┬───────────────────────────────┘
                │ invoca / configura (proceso externo)
┌───────────────▼───────────────────────────────┐
│         MOTOR (backend intercambiable)         │
│   Cordial (GPL-3.0, recomendado) | Sober       │
└───────────────┬───────────────────────────────┘
                │ carga
┌───────────────▼───────────────────────────────┐
│   APK oficial de Roblox (lib/x86_64 + assets)  │
│        suministrado por el propio usuario      │
└─────────────────────────────────────────────┘
```

## Módulos del backend Rust (`src-tauri/src/`)

- `engine/` — **implementado (MVP punto 2).** Se divide en:
  - `flatpak.rs`: wrapper genérico sobre el CLI de `flatpak` (detección,
    remotos, instalación con streaming de progreso línea por línea).
  - `cordial.rs`: constantes y lógica específicas del backend Cordial
    (remoto propio `cordial` → `https://luohoa97.github.io/cordial/cordial.flatpakrepo`,
    app id `io.github.luohoa97.Cordial`, ya que Cordial no está en Flathub).
    Desde Cordial 0.8.0 el motor descarga y verifica el build de Roblox por
    su cuenta, así que este módulo no gestiona ningún APK.
  - `commands.rs`: comandos Tauri (`engine_status`, `engine_install`,
    `engine_launch`, `engine_known_issues_url`) y los eventos que emite
    `engine_install` durante la instalación (`engine://install-log`,
    `engine://install-done`, `engine://install-error`).
  - Diseñado como capa intercambiable: un futuro backend (p. ej. Sober, si
    vuelve a ser open source) implicaría agregar `sober.rs` con la misma
    forma que `cordial.rs`, sin tocar `flatpak.rs` ni el frontend.
- `fflags/` — **implementado (MVP punto 3).** Se divide en:
  - `models.rs`: tipos compartidos — `FlagValue` (Bool/Int/Str), `FlagEntry`
    (nombre + habilitado + valor), `Preset` (conjunto con nombre, propio de
    Vaporstrap) y `CatalogEntry` (entrada del catálogo de referencia).
  - `catalog.rs`: catálogo incorporado de ~10 flags conocidas, marcado
    explícitamente como punto de partida no verificado contra la build
    actual de Roblox.
  - `document.rs`: lee/escribe el `flags.json` **del perfil de Cordial**
    (`$XDG_DATA_HOME/cordial/profiles/<nombre>/flags.json`, ruta y formato
    verificados contra `crates/cordial-plugins/src/flag_document.rs` del
    código fuente de Cordial). Ese archivo es la capa de mayor prioridad de
    Cordial — gana sobre cualquier plugin y sobre Roblox — así que
    Vaporstrap escribe ahí directamente, sin pasar por el sistema de
    plugins/capabilities de Cordial. También soporta parsear/emitir el
    mismo formato plano que usan los exports de Bloxstrap.
  - `commands.rs`: comandos Tauri para catálogo, CRUD de presets propios
    (guardados en `<app_data_dir>/presets/*.json`), import/export JSON, y
    `fflags_apply_to_cordial` (reemplaza el `flags.json` del perfil elegido
    con las flags habilitadas del preset actual).
  - Los "presets" de Vaporstrap (conjuntos con nombre que arma el usuario) y
    los "perfiles" de Cordial (cuentas/instancias) son conceptos distintos;
    ver el comentario de cabecera de `fflags/mod.rs`.
- `hardware/` — **implementado (MVP punto 4).** Se divide en:
  - `probe.rs`: sondeo best-effort (núcleos físicos vía `/proc/cpuinfo`, RAM
    vía `/proc/meminfo`, ICDs de Vulkan en las rutas estándar, GPU vía
    `lspci` si está disponible, y un microbenchmark de CPU deliberadamente
    simple y determinista).
  - `recommend.rs`: reglas simples que traducen el sondeo a una
    `Recommendation` con razonamiento en texto plano por cada decisión.
  - `settings.rs`: persiste la recomendación aplicada en
    `<app_data_dir>/launch-env.json`.
  - `commands.rs`: `hardware_probe`, `hardware_recommend`,
    `hardware_current_launch_env`, `hardware_apply`.
  - **Decisión clave:** no reimplementa tuning de rendimiento — usa
    directamente los modos nativos de Cordial, verificados contra su código
    fuente: `CORDIAL_PERFORMANCE` (`balanced`/`throughput`/`latency`, en
    `crates/cordial-runtime/src/flags.rs`) y `CORDIAL_GRAPHICS`
    (`automatic`/`vulkan`/`gles`, en `crates/cordial-runtime/src/graphics.rs`).
    `engine::cordial::launch` ahora acepta pares de entorno y los pasa como
    `flatpak run --env=CLAVE=VALOR`, que es como el propio shell de Cordial
    fija esas variables. El FPS cap y la calidad gráfica sí son FastFlags
    (`DFIntTaskSchedulerTargetFps`, `FIntRenderGraphicsQualityOverride`) y se
    aplican con `fflags::document::merge_flags` — una escritura puntual que
    no pisa un preset ya aplicado desde el editor del punto 3.
- `hardware/` — mini-benchmark de CPU/GPU/RAM y heurística de
  recomendación de perfil (resolución, FPS cap, renderer, calidad).
- `discord/` — **implementado (MVP punto 5, parte 1).** Rich Presence
  propio, sin pasar por el plugin de Cordial (ver justificación en el
  comentario de cabecera de `discord/mod.rs`, con la cita exacta de qué
  métodos responde `plugin_host.rs`). Se divide en:
  - `ipc.rs`: protocolo IPC de Discord a mano sobre `UnixStream`
    (descubrimiento de `discord-ipc-0..9` en las rutas estándar, Flatpak y
    snap; framing binario opcode+largo+JSON).
  - `client.rs`: handshake + `SET_ACTIVITY` (activar/limpiar).
  - `settings.rs`: persiste si está habilitado y el Client ID (propio del
    usuario — Vaporstrap no trae uno embebido).
  - `presence.rs`: `watch_process` espera el `Child` de Cordial en un hilo
    aparte — de paso corrige un bug que existía desde el punto 2 (el
    proceso hijo se descartaba sin esperarlo, pudiendo quedar zombie).
  - `commands.rs`: `discord_get_settings`, `discord_save_settings`,
    `discord_test`, `discord_clear`.
- `profiles/` — **implementado (MVP punto 5, parte 2).** Reformula
  "multi-instancia" como gestión de perfiles de Cordial, porque
  `cordial-shell` es una `GApplication` de ID fijo y por lo tanto de proceso
  único (ver justificación completa en `profiles/mod.rs`). Aporta creación/
  renombrado/borrado de perfiles (`fflags::document::create_profile` y
  afines) y lanzar Cordial apuntando a un link de Roblox
  (`profiles_launch_with_link`), que es el único argumento externo que el
  binario acepta.
- `logs/` — **implementado (MVP punto 6). MVP completo.** Todo local, sin
  telemetría. Se divide en:
  - `paths.rs`: rutas de `vaporstrap.log` y de un archivo nuevo por cada
    lanzamiento de Cordial (`cordial-<fecha>.log`).
  - `logger.rs`: logger simple con rotación (se archiva a `.log.old` al
    superar 2 MB).
  - `commands.rs`: listar/leer logs (con cola de hasta 512 KB para no
    tirarle archivos gigantes a la UI, y validación de nombre de archivo
    para que el frontend no pueda pedir leer una ruta arbitraria) y
    `logs_generate_diagnostic_report`, que junta specs de hardware, estado
    del motor, config de Discord (sin exponer el client_id) y las últimas
    líneas de los logs en un `.txt` — nunca se envía a ningún lado
    automáticamente.
  - `engine::cordial::launch` ahora acepta una ruta de log opcional y
    redirige ahí stdout/stderr del proceso de Cordial. Antes de este punto,
    esa salida se heredaba de Vaporstrap y se perdía en cualquier lanzador
    gráfico sin terminal — corregido acá.

## Frontend (`src/`)

Componentes Svelte organizados por sección de la app (a definir conforme
avance el MVP): panel de FastFlags, panel de optimización, gestor de
cuentas, configuración general.

## Empaquetado

- **Flatpak** (`flatpak/`): manifiesto objetivo para Flathub. El
  `finish-args` da acceso a red (para descargar/actualizar el motor y
  revisar releases) y a `dri` (aceleración gráfica).
- **AppImage**: generado vía `tauri-apps/tauri-action` en CI.

## Por qué Cordial y no reimplementar un motor

Reimplementar la capa que ejecuta el binario/APK de Roblox en Linux
implicaría reconstruir trabajo de ingeniería inversa ya hecho (linker
bionic, shims JNI, etc.) y mantenerlo contra cada actualización de Roblox.
Cordial ya resuelve eso y es GPL-3.0, compatible con la licencia de este
proyecto. Vaporstrap se enfoca en la capa que le falta a ese motor: UX,
configuración y herramientas — el mismo rol que cumple Bloxstrap sobre el
cliente oficial de Windows.
