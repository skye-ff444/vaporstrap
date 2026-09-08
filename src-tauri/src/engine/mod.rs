//! Integración con el motor que ejecuta Roblox en Linux.
//!
//! Vaporstrap no reimplementa un motor: orquesta uno externo. Por ahora el
//! único backend soportado es Cordial (GPL-3.0), pero este módulo está
//! separado en `flatpak` (genérico) y `cordial` (específico) para poder
//! añadir otro backend (p. ej. Sober, si vuelve a ser open source) sin tocar
//! el resto de la app.

pub mod commands;
pub mod cordial;
pub mod flatpak;

use serde::Serialize;

/// Estado del motor, tal como lo ve la UI.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EngineStatus {
    /// El comando `flatpak` no está disponible en el sistema.
    FlatpakMissing,
    /// Flatpak está disponible pero el remoto de Cordial no está agregado.
    RemoteMissing,
    /// El remoto está agregado pero Cordial no está instalado.
    NotInstalled,
    /// Cordial está instalado y listo para lanzarse.
    Installed { version: String },
}
