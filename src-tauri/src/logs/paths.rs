use std::path::PathBuf;

use tauri::{AppHandle, Manager};

pub fn logs_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("no se pudo resolver el directorio de datos de la app: {e}"))?
        .join("logs");
    std::fs::create_dir_all(&dir).map_err(|e| format!("no se pudo crear {}: {e}", dir.display()))?;
    Ok(dir)
}

pub fn vaporstrap_log_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(logs_dir(app)?.join("vaporstrap.log"))
}

/// Una ruta nueva y única para el log de un lanzamiento de Cordial que está
/// por arrancar. No pisa lanzamientos anteriores.
pub fn new_cordial_log_path(app: &AppHandle) -> Result<PathBuf, String> {
    let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S%.3f");
    Ok(logs_dir(app)?.join(format!("cordial-{timestamp}.log")))
}
