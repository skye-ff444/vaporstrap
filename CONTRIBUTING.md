# Contribuir a Vaporstrap

¡Gracias por tu interés en contribuir! Antes de nada, lee el
[README](README.md), especialmente el aviso legal.

## Requisitos previos

- Rust (stable) + [prerequisitos de Tauri v2 para Linux](https://v2.tauri.app/start/prerequisites/#linux)
- Node.js 18+
- Familiaridad básica con Svelte y con el sistema de FastFlags de Roblox
  ayuda, pero no es obligatoria para empezar.

## Flujo de trabajo

1. Haz fork del repositorio y crea una rama descriptiva
   (`feature/editor-fflags`, `fix/deteccion-apk`, etc.).
2. Corre `npm install` y `npm run tauri dev` para levantar el entorno.
3. Antes de abrir un PR:
   - `cargo fmt --manifest-path src-tauri/Cargo.toml`
   - `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
4. Describe en el PR qué problema resuelve y, si aplica, cómo lo probaste.

## Alcance del proyecto

- Vaporstrap **orquesta** motores externos (Cordial, y en el futuro
  posiblemente Sober); no reimplementa un motor de Roblox.
- No se aceptarán contribuciones que:
  - distribuyan APKs, binarios o assets de Roblox;
  - añadan funcionalidad de tipo "executor" o bypass de anti-cheat
    (Byfron/Hyperion) — eso está fuera de alcance de este proyecto por
    completo.

## Reportar bugs / pedir features

Usa las plantillas de [Issues](../../issues/new/choose).

## Seguridad

Para vulnerabilidades, sigue el proceso en [`SECURITY.md`](SECURITY.md) en
vez de abrir un issue público.
