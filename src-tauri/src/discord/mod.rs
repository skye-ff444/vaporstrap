//! Discord Rich Presence, implementado directamente por Vaporstrap.
//!
//! **Por qué no usamos el plugin que trae Cordial:** Cordial incluye un
//! plugin de presencia de Discord de fábrica (`plugins/discord-presence/`),
//! con el protocolo IPC de Discord y el framing ya resueltos y probados. Pero
//! verificado contra el código fuente de Cordial al momento de escribir esto
//! (`crates/cordial-runtime/src/plugin_host.rs`), el host de plugins que
//! corre el cliente solo implementa los métodos `settings.*`, `flags.*` y
//! `log.write` — todo lo demás, incluida la suscripción a eventos de ciclo
//! de vida que ese plugin necesita para saber cuándo Roblox arrancó o a qué
//! experiencia se unió, cae en un catch-all que responde literalmente
//! "is not implemented yet". Un plugin de presencia con permisos completos
//! arrancaría y quedaría esperando eventos que nunca llegan.
//!
//! Por eso esta implementación:
//! - No pasa por el sistema de plugins de Cordial en absoluto.
//! - No sabe (ni puede saber, por ahora) a qué experiencia de Roblox está
//!   jugando el usuario — esa información solo la tiene el plugin de Cordial,
//!   justamente por la vía que no funciona todavía.
//! - Se limita a marcar "hay una sesión de Cordial corriendo, lanzada desde
//!   Vaporstrap" con la hora de inicio, y la limpia cuando el proceso termina.
//!
//! Si Cordial completa esa pieza de su plugin host más adelante, esto podría
//! simplificarse o incluso delegarse por completo a su propio plugin.
//!
//! **Requiere un Client ID de Discord propio.** Vaporstrap no trae uno
//! embebido: no es nuestro para reclamarlo, y uno inventado simplemente
//! fallaría en silencio. Cualquiera puede crear uno gratis en
//! <https://discord.com/developers/applications> (2 minutos) y pegarlo en
//! la configuración de este módulo.

pub mod client;
pub mod commands;
pub mod ipc;
pub mod presence;
pub mod settings;
