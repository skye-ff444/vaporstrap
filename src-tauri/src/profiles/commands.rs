//! Comandos IPC para gestionar perfiles de Cordial y lanzar con un link.

use tauri::AppHandle;

use crate::discord::presence;
use crate::engine::cordial;
use crate::fflags::document;

#[tauri::command]
pub fn profiles_list() -> Result<Vec<String>, String> {
    document::list_profiles()
}

#[tauri::command]
pub fn profiles_create(name: String) -> Result<(), String> {
    document::create_profile(&name)
}

#[tauri::command]
pub fn profiles_delete(name: String) -> Result<(), String> {
    document::delete_profile(&name)
}

#[tauri::command]
pub fn profiles_rename(old_name: String, new_name: String) -> Result<(), String> {
    document::rename_profile(&old_name, &new_name)
}

/// Lanza Cordial apuntando directo a un link de Roblox (`roblox-player:...`
/// o `roblox:...`), el único argumento posicional que el binario acepta
/// desde afuera. Si Cordial ya está corriendo, esto despierta esa instancia
/// y le entrega el link — no abre un proceso nuevo (ver doc de `super`).
#[tauri::command]
pub fn profiles_launch_with_link(app: AppHandle, link: String) -> Result<(), String> {
    if link.trim().is_empty() {
        return Err("el link no puede estar vacío".to_string());
    }

    let env = crate::hardware::settings::load(&app);
    let log_path = crate::logs::paths::new_cordial_log_path(&app).ok();

    let child = cordial::launch(&[link.clone()], &env.to_pairs(), log_path.as_deref())?;
    crate::logs::logger::info(&app, &format!("Cordial lanzado con link: {link}"));

    presence::watch_process(app, child);
    Ok(())
}
