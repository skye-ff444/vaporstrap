//! Comandos IPC para el visor de logs y el reporte de diagnóstico.

use std::fs;

use serde::Serialize;
use tauri::AppHandle;

use super::paths;

#[derive(Debug, Clone, Serialize)]
pub struct LogFileInfo {
    pub name: String,
    pub modified_unix: i64,
    pub size_bytes: u64,
}

const MAX_READ_BYTES: u64 = 512 * 1024; // 512 KB — suficiente para diagnosticar sin tirarle un archivo gigante a la UI

fn read_tail(path: &std::path::Path) -> Result<String, String> {
    if !path.exists() {
        return Ok(String::new());
    }

    let meta = fs::metadata(path).map_err(|e| format!("no se pudo leer {}: {e}", path.display()))?;
    let text = fs::read_to_string(path).map_err(|e| format!("no se pudo leer {}: {e}", path.display()))?;

    if meta.len() > MAX_READ_BYTES {
        let start = text.len().saturating_sub(MAX_READ_BYTES as usize);
        Ok(format!("… (truncado, mostrando el final)\n{}", &text[start..]))
    } else {
        Ok(text)
    }
}

#[tauri::command]
pub fn logs_list_cordial_launches(app: AppHandle) -> Result<Vec<LogFileInfo>, String> {
    let dir = paths::logs_dir(&app)?;
    let entries = fs::read_dir(&dir).map_err(|e| format!("no se pudo leer {}: {e}", dir.display()))?;

    let mut files: Vec<LogFileInfo> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|n| n.starts_with("cordial-") && n.ends_with(".log"))
                .unwrap_or(false)
        })
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            let modified = meta
                .modified()
                .ok()?
                .duration_since(std::time::UNIX_EPOCH)
                .ok()?
                .as_secs() as i64;
            Some(LogFileInfo {
                name: e.file_name().to_string_lossy().to_string(),
                modified_unix: modified,
                size_bytes: meta.len(),
            })
        })
        .collect();

    files.sort_by(|a, b| b.modified_unix.cmp(&a.modified_unix));
    Ok(files)
}

/// Lee un log por nombre de archivo (no ruta completa) dentro del
/// directorio de logs de Vaporstrap — así el frontend no puede pedir leer
/// un archivo arbitrario del sistema.
#[tauri::command]
pub fn logs_read(app: AppHandle, name: String) -> Result<String, String> {
    if name.contains('/') || name.contains("..") {
        return Err("nombre de archivo inválido".to_string());
    }
    read_tail(&paths::logs_dir(&app)?.join(&name))
}

#[tauri::command]
pub fn logs_read_vaporstrap(app: AppHandle) -> Result<String, String> {
    read_tail(&paths::vaporstrap_log_path(&app)?)
}

#[tauri::command]
pub fn logs_dir_path(app: AppHandle) -> Result<String, String> {
    Ok(paths::logs_dir(&app)?.to_string_lossy().to_string())
}

/// Arma un reporte de diagnóstico local: versión, specs de hardware, estado
/// del motor, config de Discord (sin el client_id en texto plano) y las
/// últimas líneas de los logs. Se guarda en disco y listo — nunca se envía
/// a ningún lado automáticamente; el usuario decide qué hacer con él.
#[tauri::command]
pub fn logs_generate_diagnostic_report(app: AppHandle) -> Result<String, String> {
    let hardware = crate::hardware::probe::snapshot();
    let engine_status = crate::engine::cordial::status();
    let discord = crate::discord::settings::load(&app);

    let vaporstrap_log = read_tail(&paths::vaporstrap_log_path(&app)?).unwrap_or_default();

    let launches = logs_list_cordial_launches(app.clone())?;
    let last_launch_log = if let Some(latest) = launches.first() {
        read_tail(&paths::logs_dir(&app)?.join(&latest.name)).unwrap_or_default()
    } else {
        "(sin lanzamientos registrados todavía)".to_string()
    };

    let report = format!(
        "=== Reporte de diagnóstico de Vaporstrap ===\n\
         Generado: {}\n\
         Versión de Vaporstrap: {}\n\
         \n\
         --- Hardware ---\n\
         {:#?}\n\
         \n\
         --- Estado del motor (Cordial) ---\n\
         {:?}\n\
         \n\
         --- Discord RPC ---\n\
         habilitado: {}, client_id configurado: {}\n\
         \n\
         --- Log de Vaporstrap (cola, hasta 512 KB) ---\n\
         {}\n\
         \n\
         --- Último log de lanzamiento de Cordial (cola, hasta 512 KB) ---\n\
         {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        env!("CARGO_PKG_VERSION"),
        hardware,
        engine_status,
        discord.enabled,
        if discord.client_id.as_deref().unwrap_or("").trim().is_empty() {
            "no"
        } else {
            "sí"
        },
        vaporstrap_log,
        last_launch_log,
    );

    let path = paths::logs_dir(&app)?.join(format!(
        "diagnostico-{}.txt",
        chrono::Local::now().format("%Y%m%d-%H%M%S")
    ));

    fs::write(&path, &report).map_err(|e| format!("no se pudo guardar {}: {e}", path.display()))?;

    Ok(path.to_string_lossy().to_string())
}
