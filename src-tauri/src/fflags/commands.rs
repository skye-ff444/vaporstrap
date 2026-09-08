//! Comandos IPC para el editor de FastFlags.
//!
//! Los "presets" de Vaporstrap se guardan como archivos JSON individuales en
//! el directorio de datos de la app (`<app_data_dir>/presets/<nombre>.json`),
//! cada uno serializando directamente un `Preset`. No hay base de datos: son
//! pocos archivos chicos y el usuario puede abrir la carpeta y mirarlos.

use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use super::catalog::builtin_catalog;
use super::document;
use super::models::{CatalogEntry, FlagEntry, Preset};

fn presets_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("no se pudo resolver el directorio de datos de la app: {e}"))?
        .join("presets");
    std::fs::create_dir_all(&dir).map_err(|e| format!("no se pudo crear {}: {e}", dir.display()))?;
    Ok(dir)
}

fn preset_path(app: &AppHandle, name: &str) -> Result<PathBuf, String> {
    // Nombre de archivo simple; el editor no expone caracteres raros en el
    // nombre del preset, pero igual lo saneamos por si se importa uno.
    let safe_name: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    Ok(presets_dir(app)?.join(format!("{safe_name}.json")))
}

#[tauri::command]
pub fn fflags_catalog() -> Vec<CatalogEntry> {
    builtin_catalog()
}

#[tauri::command]
pub fn fflags_list_presets(app: AppHandle) -> Result<Vec<String>, String> {
    let dir = presets_dir(&app)?;
    let entries =
        std::fs::read_dir(&dir).map_err(|e| format!("no se pudo leer {}: {e}", dir.display()))?;

    let mut names: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .filter_map(|e| e.path().file_stem().and_then(|s| s.to_str()).map(String::from))
        .collect();
    names.sort();
    Ok(names)
}

#[tauri::command]
pub fn fflags_load_preset(app: AppHandle, name: String) -> Result<Preset, String> {
    let path = preset_path(&app, &name)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("no se pudo leer el preset '{name}': {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("preset '{name}' corrupto: {e}"))
}

#[tauri::command]
pub fn fflags_save_preset(app: AppHandle, preset: Preset) -> Result<(), String> {
    let path = preset_path(&app, &preset.name)?;
    let text = serde_json::to_string_pretty(&preset)
        .map_err(|e| format!("no se pudo serializar el preset: {e}"))?;
    std::fs::write(&path, text).map_err(|e| format!("no se pudo guardar {}: {e}", path.display()))
}

#[tauri::command]
pub fn fflags_delete_preset(app: AppHandle, name: String) -> Result<(), String> {
    let path = preset_path(&app, &name)?;
    std::fs::remove_file(&path).map_err(|e| format!("no se pudo borrar el preset '{name}': {e}"))
}

/// Importa un JSON plano (propio o export de Bloxstrap) como una lista de
/// flags *sin guardar* — el frontend decide si guardarlo como preset y con
/// qué nombre.
#[tauri::command]
pub fn fflags_import_json(text: String) -> Result<Vec<FlagEntry>, String> {
    document::parse_flat_json(&text)
}

/// Exporta las flags habilitadas al mismo formato plano.
#[tauri::command]
pub fn fflags_export_json(entries: Vec<FlagEntry>) -> Result<String, String> {
    document::to_flat_json(&entries)
}

/// Perfiles de Cordial detectados en el sistema (para elegir a cuál aplicar).
#[tauri::command]
pub fn fflags_list_cordial_profiles() -> Result<Vec<String>, String> {
    document::list_profiles()
}

/// Lee las flags actualmente escritas en el `flags.json` de un perfil de
/// Cordial (para mostrar "esto es lo que ya está aplicado" antes de pisarlo).
#[tauri::command]
pub fn fflags_read_cordial_profile(
    profile: String,
) -> Result<std::collections::BTreeMap<String, serde_json::Value>, String> {
    document::read_flags(&profile)
}

/// Escribe las flags habilitadas de `entries` en el `flags.json` del perfil
/// de Cordial indicado, reemplazando lo que hubiera antes.
#[tauri::command]
pub fn fflags_apply_to_cordial(
    app: AppHandle,
    profile: String,
    entries: Vec<FlagEntry>,
) -> Result<(), String> {
    let count = entries.iter().filter(|e| e.enabled).count();
    document::write_flags(&profile, &entries)?;
    crate::logs::logger::info(
        &app,
        &format!("Preset de FastFlags aplicado al perfil '{profile}' ({count} flags habilitadas)."),
    );
    Ok(())
}
