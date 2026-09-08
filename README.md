# Vaporstrap

**Bootstrapper/lanzador de código abierto para jugar Roblox en Linux.**

Vaporstrap añade una capa de configuración (editor de FastFlags, perfiles de
rendimiento, mods, Discord RPC, multi-cuenta) sobre un motor externo que
ejecuta Roblox de forma nativa en Linux. Está inspirado en
[Bloxstrap](https://github.com/bloxstraplabs/bloxstrap),
[Voidstrap](https://github.com/pizzaboxer/voidstrap),
[Fishstrap](https://github.com/returnity/fishstrap) (Windows) y
[AppleBlox](https://github.com/AppleBlox/AppleBlox) (macOS).

Vaporstrap **no** es un motor nuevo ni reimplementa Roblox: orquesta y
configura un motor externo (actualmente [Cordial](https://github.com/luohoa97/cordial),
GPL-3.0) que carga `libroblox.so` de forma nativa, sin Wine, sin emulador y
sin máquina virtual. Cordial no está en Flathub (por la política de Flathub
sobre contenido asistido por IA), así que Vaporstrap agrega su remoto de
Flatpak propio (`cordial-flatpakrepo`) automáticamente antes de instalarlo.

---

## ⚠️ Aviso legal y de alcance

- **Este es un proyecto no oficial**, no afiliado, respaldado ni aprobado
  por Roblox Corporation. Roblox® es una marca registrada de Roblox
  Corporation.
- Los Términos de Servicio de Roblox **prohíben el uso de clientes o
  lanzadores no oficiales**. Usar Vaporstrap puede infringir esos términos y
  derivar en la suspensión de tu cuenta. **Lo usas bajo tu propio riesgo.**
- Vaporstrap **no distribuye** el instalador de Roblox, el APK, ni ningún
  asset propiedad de Roblox Corporation. Debes obtenerlos por tu cuenta.
- Vaporstrap **no es un "executor" ni un bypass de anti-cheat** (Byfron/Hyperion).
  No incluye, ni incluirá, funcionalidad de ese tipo. Eso pertenece a otra
  categoría de software completamente distinta, mucho más grave en términos
  de ToS y con riesgos serios de baneo permanente y malware para quien lo usa.
- Este proyecto solo orquesta/configura software de terceros ya existente;
  no reclama ninguna afiliación con Roblox Corporation ni con los mantenedores
  de los motores que integra.

---

## 🧩 Arquitectura

```
┌─────────────────────────────────────────────┐
│                 VAPORSTRAP (GUI)             │
│  - Editor de FastFlags                       │
│  - Perfiles de optimización por hardware     │
│  - Gestor de mods / temas / canales          │
│  - Multi-instancia, Discord RPC, logs        │
└───────────────┬───────────────────────────────┘
                │ invoca / configura
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

Más detalle en [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

## 🛠️ Stack

- **Backend:** Rust + [Tauri v2](https://tauri.app)
- **Frontend:** Svelte
- **Empaquetado:** Flatpak (candidato a Flathub) + AppImage
- **CI/CD:** GitHub Actions

## ⚠️ Limitaciones conocidas de Cordial (no de Vaporstrap)

Verificadas contra el código fuente de Cordial al integrar los puntos 2 y 5
del MVP:

- **Discord Rich Presence de Cordial no funciona todavía.** Trae su propio
  plugin (`plugins/discord-presence/`), pero el host de plugins del cliente
  (`crates/cordial-runtime/src/plugin_host.rs`) solo implementa `settings.*`,
  `flags.*` y `log.write` — la suscripción a eventos de ciclo de vida que ese
  plugin necesita responde "is not implemented yet". Por eso Vaporstrap
  implementa su propio Rich Presence básico (sin detalle de a qué
  experiencia estás jugando, esa info no está disponible por ningún canal
  externo todavía).
- **Cordial es de proceso único** (GApplication de ID fijo): no se puede
  abrir "una segunda instancia" desde afuera con un segundo `flatpak run`.
  El multi-perfil real ocurre dentro de la propia interfaz de Cordial, con
  su selector de perfiles.

## 📦 Estado del proyecto

🚧 En desarrollo activo — MVP en construcción. Ver el roadmap en
[Issues](../../issues) y en `docs/`.

### Roadmap del MVP

1. [x] Estructura base del proyecto, licencia, README, CI
2. [x] Capa de integración con el motor (detección/instalación de Cordial vía su Flatpak propio, lanzamiento). Desde Cordial 0.8.0 el propio motor descarga y verifica el build de Roblox, así que Vaporstrap no gestiona el APK del usuario.
3. [x] Editor de FastFlags (buscar, activar/desactivar, import/export JSON, presets, aplicar a un perfil de Cordial)
4. [x] Módulo de optimización automática: sondea CPU/RAM/Vulkan y recomienda uno de los modos nativos de Cordial (`CORDIAL_PERFORMANCE`, `CORDIAL_GRAPHICS`) más FPS cap y calidad gráfica
5. [x] Discord Rich Presence (implementación propia, ver limitación abajo) y gestión de perfiles de Cordial
6. [x] Visor de logs y manejo de errores (todo local, sin telemetría)

**MVP completo.** Las ideas de v2 (mods, temas, plugins, empaquetado en AUR/Flathub/Nixpkgs, Wayland nativo) quedan para más adelante — ver [Issues](../../issues).

## 🚀 Instalación (desarrollo)

Requisitos:

- [Rust](https://www.rust-lang.org/tools/install) (stable) + toolchain de Tauri
- [Node.js](https://nodejs.org/) 18+
- Dependencias de sistema de Tauri para Linux: ver
  [prerequisitos de Tauri v2](https://v2.tauri.app/start/prerequisites/#linux)
  (webkit2gtk, libayatana-appindicator, etc.)

```bash
git clone https://github.com/<tu-usuario>/vaporstrap.git
cd vaporstrap
npm install
npm run tauri dev
```

## 📥 Instalación (usuario final)

Aún no hay releases estables. Cuando existan, se publicarán como Flatpak y
AppImage en la sección [Releases](../../releases).

## 🤝 Contribuir

Ver [`CONTRIBUTING.md`](CONTRIBUTING.md). Reportes de seguridad: ver
[`SECURITY.md`](SECURITY.md).

## 📄 Licencia

[GPL-3.0](LICENSE) — coherente con Bloxstrap, Voidstrap y Cordial, y
garantiza que los forks permanezcan abiertos.
