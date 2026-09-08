//! Acceso al `flags.json` de un **perfil de Cordial** (no confundir con los
//! "presets" de Vaporstrap, ver `super`).
//!
//! Rutas verificadas contra el código fuente de Cordial:
//! - Perfiles: `$XDG_DATA_HOME/cordial/profiles/<nombre>/` (ADR-012). El
//!   perfil por defecto se llama literalmente `default` (ADR-013).
//! - Flags del perfil: `<profile_dir>/flags.json`
//!   (`crates/cordial-plugins/src/flag_document.rs::path_in`), salvo que la
//!   variable de entorno `CORDIAL_FLAGS` la redirija — eso es un switch de
//!   desarrollo de Cordial, no algo que Vaporstrap deba tocar.

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::models::{FlagEntry, FlagValue};

/// `$XDG_DATA_HOME`, o `~/.local/share` si no está seteada (igual que hace
/// Cordial y cualquier app conforme a XDG Base Directory).
fn xdg_data_home() -> Result<PathBuf, String> {
    if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
        if !dir.is_empty() {
            return Ok(PathBuf::from(dir));
        }
    }
    let home = std::env::var("HOME").map_err(|_| "no se pudo determinar $HOME".to_string())?;
    Ok(PathBuf::from(home).join(".local/share"))
}

fn cordial_profiles_dir() -> Result<PathBuf, String> {
    Ok(xdg_data_home()?.join("cordial").join("profiles"))
}

/// Lista los perfiles de Cordial existentes. Vacío (no error) si Cordial
/// nunca se abrió todavía.
pub fn list_profiles() -> Result<Vec<String>, String> {
    let dir = cordial_profiles_dir()?;
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let entries = std::fs::read_dir(&dir)
        .map_err(|e| format!("no se pudo leer {}: {e}", dir.display()))?;

    let mut profiles: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    profiles.sort();
    Ok(profiles)
}

fn flags_path_for(profile: &str) -> Result<PathBuf, String> {
    Ok(cordial_profiles_dir()?.join(profile).join("flags.json"))
}

/// Ruta del directorio de un perfil de Cordial (no implica que exista).
pub fn profile_dir(profile: &str) -> Result<PathBuf, String> {
    Ok(cordial_profiles_dir()?.join(profile))
}

/// Crea el directorio de un perfil nuevo. Solo crea la carpeta: Cordial
/// completa el resto (sesión, configuración) la primera vez que alguien lo
/// abre desde el propio selector de perfiles de Cordial. Falla si ya existe.
pub fn create_profile(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("el nombre del perfil no puede estar vacío".to_string());
    }
    let dir = profile_dir(name)?;
    if dir.exists() {
        return Err(format!("el perfil '{name}' ya existe"));
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("no se pudo crear {}: {e}", dir.display()))
}

/// Borra un perfil por completo: su sesión, sus flags y los datos de Roblox
/// asociados a esa cuenta. No comprueba si Cordial lo tiene abierto en ese
/// momento — la UI debe advertirlo antes de llamar a esto.
pub fn delete_profile(name: &str) -> Result<(), String> {
    let dir = profile_dir(name)?;
    if !dir.exists() {
        return Err(format!("el perfil '{name}' no existe"));
    }
    std::fs::remove_dir_all(&dir).map_err(|e| format!("no se pudo borrar {}: {e}", dir.display()))
}

pub fn rename_profile(old_name: &str, new_name: &str) -> Result<(), String> {
    if new_name.trim().is_empty() {
        return Err("el nuevo nombre no puede estar vacío".to_string());
    }
    let old_dir = profile_dir(old_name)?;
    let new_dir = profile_dir(new_name)?;
    if !old_dir.exists() {
        return Err(format!("el perfil '{old_name}' no existe"));
    }
    if new_dir.exists() {
        return Err(format!("ya existe un perfil llamado '{new_name}'"));
    }
    std::fs::rename(&old_dir, &new_dir)
        .map_err(|e| format!("no se pudo renombrar '{old_name}' a '{new_name}': {e}"))
}

/// Lee el `flags.json` de un perfil. Si el perfil o el archivo no existen
/// todavía, devuelve un mapa vacío (no es un error: significa "sin
/// overrides propios puestos").
pub fn read_flags(profile: &str) -> Result<BTreeMap<String, serde_json::Value>, String> {
    let path = flags_path_for(profile)?;
    if !path.exists() {
        return Ok(BTreeMap::new());
    }

    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("no se pudo leer {}: {e}", path.display()))?;

    if text.trim().is_empty() {
        return Ok(BTreeMap::new());
    }

    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("{} no es JSON válido: {e}", path.display()))?;

    value
        .as_object()
        .map(|obj| obj.clone().into_iter().collect())
        .ok_or_else(|| format!("{} debe ser un objeto JSON de flag -> valor", path.display()))
}

/// Escribe (reemplaza por completo) el `flags.json` de un perfil con las
/// flags **habilitadas** de `entries`. Las deshabilitadas se omiten del
/// archivo, no se escriben con un valor "apagado" — Cordial simplemente no
/// las ve como override si no están.
///
/// Esto sobreescribe cualquier flag que el archivo tuviera antes y que no
/// venga de Vaporstrap. Es una decisión de diseño deliberada (que Vaporstrap
/// sea la única fuente de verdad de ese archivo mientras se use), y la UI
/// debe avisarlo antes de llamar a esto.
pub fn write_flags(profile: &str, entries: &[FlagEntry]) -> Result<(), String> {
    let path = flags_path_for(profile)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("no se pudo crear {}: {e}", parent.display()))?;
    }

    let mut map = serde_json::Map::new();
    for entry in entries.iter().filter(|e| e.enabled) {
        map.insert(entry.name.clone(), entry.value.to_json());
    }

    let text = serde_json::to_string_pretty(&serde_json::Value::Object(map))
        .map_err(|e| format!("no se pudo serializar las flags: {e}"))?;

    std::fs::write(&path, text).map_err(|e| format!("no se pudo escribir {}: {e}", path.display()))
}

/// Parsea texto JSON plano (`{"Flag": valor, ...}`) a una lista de
/// `FlagEntry`, todas habilitadas. Acepta tal cual el formato de export de
/// Bloxstrap, según confirma el propio código de Cordial.
pub fn parse_flat_json(text: &str) -> Result<Vec<FlagEntry>, String> {
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }

    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("JSON inválido: {e}"))?;

    let obj = value
        .as_object()
        .ok_or_else(|| "el documento debe ser un objeto JSON de flag -> valor".to_string())?;

    let mut entries = Vec::with_capacity(obj.len());
    for (name, raw_value) in obj {
        if name.trim().is_empty() {
            return Err("hay una flag con nombre vacío".to_string());
        }
        if raw_value.is_null() || raw_value.is_array() || raw_value.is_object() {
            return Err(format!(
                "{name}: el valor debe ser texto, número o true/false"
            ));
        }
        entries.push(FlagEntry {
            name: name.clone(),
            enabled: true,
            value: FlagValue::from_json(raw_value),
            description: None,
        });
    }

    Ok(entries)
}

/// Serializa una lista de flags **habilitadas** al mismo formato plano.
pub fn to_flat_json(entries: &[FlagEntry]) -> Result<String, String> {
    let mut map = serde_json::Map::new();
    for entry in entries.iter().filter(|e| e.enabled) {
        map.insert(entry.name.clone(), entry.value.to_json());
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(map))
        .map_err(|e| format!("no se pudo serializar las flags: {e}"))
}

/// Actualiza puntualmente el `flags.json` de un perfil: sobreescribe solo
/// las claves de `updates`, dejando intacto todo lo demás que ya hubiera en
/// el archivo (a diferencia de [`write_flags`], que reemplaza el documento
/// entero). Pensado para módulos que aportan un puñado de flags propias
/// (p. ej. optimización de hardware) sin pisar un preset que el usuario ya
/// aplicó desde el editor de FastFlags.
pub fn merge_flags(profile: &str, updates: &[(String, serde_json::Value)]) -> Result<(), String> {
    let mut current = read_flags(profile)?;
    for (key, value) in updates {
        current.insert(key.clone(), value.clone());
    }

    let path = flags_path_for(profile)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("no se pudo crear {}: {e}", parent.display()))?;
    }

    let text = serde_json::to_string_pretty(&serde_json::Value::Object(current.into_iter().collect()))
        .map_err(|e| format!("no se pudo serializar las flags: {e}"))?;

    std::fs::write(&path, text).map_err(|e| format!("no se pudo escribir {}: {e}", path.display()))
}
