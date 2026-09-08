//! Editor de FastFlags de Roblox.
//!
//! Terminología para no confundir dos cosas que se llaman parecido:
//! - **Preset** (de Vaporstrap): un conjunto de flags con nombre que el
//!   usuario arma, guarda, importa o exporta. Vive en el directorio de datos
//!   de Vaporstrap.
//! - **Perfil** (de Cordial): un directorio de cuenta/instancia de Cordial
//!   (`$XDG_DATA_HOME/cordial/profiles/<nombre>/`). Cada uno tiene su propio
//!   `flags.json`, que es la capa de **mayor prioridad** — gana sobre
//!   cualquier flag que aporte un plugin o el propio Roblox.
//!
//! Aplicar un preset = escribir sus flags habilitados directamente en el
//! `flags.json` del perfil de Cordial elegido. No pasa por el sistema de
//! plugins/capabilities de Cordial (que existe para que *terceros* aporten
//! flags de forma auditable) porque Vaporstrap es la propia herramienta que
//! el usuario eligió para gestionar sus flags — el mismo rol que cumple
//! Bloxstrap escribiendo su `ClientAppSettings.json` directamente.
//!
//! Formato verificado contra el código fuente de Cordial
//! (`crates/cordial-plugins/src/flag_document.rs`): un objeto JSON plano de
//! `"NombreDeFlag": valor`, donde el valor puede ser string, booleano o
//! número (Cordial lo convierte a string internamente porque así es como
//! Roblox los espera). El propio código de Cordial dice explícitamente que
//! los exports de Bloxstrap "pegan sin cambios" — por eso el import/export
//! de acá usa el mismo formato plano.

pub mod catalog;
pub mod commands;
pub mod document;
pub mod models;
