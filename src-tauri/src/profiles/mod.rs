//! Gestión de perfiles de Cordial — el aporte de Vaporstrap al punto 5 del
//! MVP para lo que el doc original llamaba "multi-instancia / multi-cuenta".
//!
//! **Por qué no es "lanzar varios `flatpak run` en paralelo":** verificado
//! contra el código fuente de Cordial (`crates/cordial-shell/src/main.rs`),
//! `cordial-shell` es una `GApplication` con un ID de aplicación fijo, lo
//! que lo hace *single-instance* a nivel de proceso: un segundo
//! `flatpak run io.github.luohoa97.Cordial` no abre una ventana nueva, sino
//! que despierta al proceso que ya está corriendo (vía D-Bus) y ese proceso
//! solo acepta ahí un link de Roblox como argumento — no un nombre de
//! perfil. El "multi-instancia" real de Cordial (una instancia = una
//! ventana, según su ADR-012) pasa por el selector de perfiles **de la
//! propia interfaz de Cordial**, dentro de ese único proceso.
//!
//! Por eso Vaporstrap no intenta orquestar procesos paralelos por perfil.
//! Lo que sí aporta:
//! - Gestión de los directorios de perfil (crear, renombrar, borrar) para
//!   que armar una cuenta nueva no dependa de tocar `~/.local/share/cordial`
//!   a mano.
//! - Lanzar Cordial apuntando directo a un link de Roblox
//!   (`roblox-player:...` / `roblox:...`), que sí es un argumento soportado
//!   externamente.
//! Elegir qué ventana usa qué perfil sigue siendo trabajo del selector de
//! perfiles dentro de Cordial.

pub mod commands;
