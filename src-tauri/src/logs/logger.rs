//! Log propio de Vaporstrap (no de Cordial — ver `mod.rs`). Un archivo de
//! texto plano, con rotación simple para que no crezca sin límite.

use std::io::Write;

use tauri::AppHandle;

use super::paths;

const MAX_LOG_BYTES: u64 = 2 * 1024 * 1024; // 2 MB

/// Nunca falla de forma visible: si no se puede escribir el log, se pierde
/// esa línea y ya — un logger que puede tirar abajo la operación que está
/// registrando sería peor que no tener logger.
pub fn append_event(app: &AppHandle, level: &str, message: &str) {
    let Ok(path) = paths::vaporstrap_log_path(app) else {
        return;
    };

    if let Ok(meta) = std::fs::metadata(&path) {
        if meta.len() > MAX_LOG_BYTES {
            let _ = std::fs::rename(&path, path.with_extension("log.old"));
        }
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let line = format!("[{timestamp}] {level:>5} {message}\n");

    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = file.write_all(line.as_bytes());
    }
}

pub fn info(app: &AppHandle, message: &str) {
    append_event(app, "INFO", message);
}

pub fn error(app: &AppHandle, message: &str) {
    append_event(app, "ERROR", message);
}
