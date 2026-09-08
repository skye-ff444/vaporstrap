//! Configuración de Discord RPC: si está habilitado, y el Client ID que el
//! usuario haya registrado en <https://discord.com/developers/applications>.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordSettings {
    pub enabled: bool,
    /// Client ID de una aplicación de Discord propia del usuario. `None` u
    /// vacío significa "no configurado" — en ese caso no se intenta nada,
    /// aunque `enabled` sea `true`.
    pub client_id: Option<String>,
}

impl Default for DiscordSettings {
    fn default() -> Self {
        DiscordSettings {
            enabled: false,
            client_id: None,
        }
    }
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("no se pudo resolver el directorio de datos de la app: {e}"))?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("no se pudo crear {}: {e}", dir.display()))?;
    Ok(dir.join("discord.json"))
}

/// Nunca falla: sin archivo o con uno corrupto, devuelve el default
/// (deshabilitado).
pub fn load(app: &AppHandle) -> DiscordSettings {
    let Ok(path) = settings_path(app) else {
        return DiscordSettings::default();
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        return DiscordSettings::default();
    };
    serde_json::from_str(&text).unwrap_or_default()
}

pub fn save(app: &AppHandle, settings: &DiscordSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    let text = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("no se pudo serializar la configuración de Discord: {e}"))?;
    std::fs::write(&path, text).map_err(|e| format!("no se pudo guardar {}: {e}", path.display()))
}
