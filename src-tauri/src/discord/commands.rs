//! Comandos IPC para configurar y probar Discord RPC.

use tauri::AppHandle;

use super::client;
use super::settings::{self, DiscordSettings};

#[tauri::command]
pub fn discord_get_settings(app: AppHandle) -> DiscordSettings {
    settings::load(&app)
}

#[tauri::command]
pub fn discord_save_settings(app: AppHandle, new_settings: DiscordSettings) -> Result<(), String> {
    settings::save(&app, &new_settings)
}

fn require_client_id(app: &AppHandle) -> Result<String, String> {
    settings::load(app)
        .client_id
        .filter(|id| !id.trim().is_empty())
        .ok_or_else(|| {
            "Falta configurar un Client ID de Discord (creá una app gratis en discord.com/developers/applications)."
                .to_string()
        })
}

/// Activa una actividad de prueba de inmediato, sin esperar a que Cordial
/// esté corriendo — útil para confirmar que el Client ID funciona.
#[tauri::command]
pub fn discord_test(app: AppHandle) -> Result<(), String> {
    let client_id = require_client_id(&app)?;
    let start = client::now_unix_secs();
    client::set_activity(&client_id, "Probando Vaporstrap", "Esto es una prueba", start)
}

#[tauri::command]
pub fn discord_clear(app: AppHandle) -> Result<(), String> {
    let client_id = require_client_id(&app)?;
    client::clear_activity(&client_id)
}
