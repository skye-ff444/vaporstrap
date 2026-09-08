//! Guarda qué recomendación de rendimiento/gráficos eligió el usuario, para
//! que `engine::commands::engine_launch` la use la próxima vez que arranque
//! Cordial sin tener que volver a pasar por esta pantalla.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LaunchEnv {
    /// "balanced" | "throughput" | "latency", o `None` si nunca se aplicó
    /// nada (Cordial usa su propio default: balanced).
    pub performance: Option<String>,
    /// "automatic" | "gles", o `None` (Cordial usa su default: automatic).
    pub graphics: Option<String>,
}

impl LaunchEnv {
    /// Como pares `(CORDIAL_PERFORMANCE, valor)` / `(CORDIAL_GRAPHICS, valor)`,
    /// listos para pasarle a `engine::cordial::launch`. Vacío si no hay nada
    /// guardado todavía.
    pub fn to_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        if let Some(performance) = &self.performance {
            pairs.push(("CORDIAL_PERFORMANCE".to_string(), performance.clone()));
        }
        if let Some(graphics) = &self.graphics {
            pairs.push(("CORDIAL_GRAPHICS".to_string(), graphics.clone()));
        }
        pairs
    }
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("no se pudo resolver el directorio de datos de la app: {e}"))?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("no se pudo crear {}: {e}", dir.display()))?;
    Ok(dir.join("launch-env.json"))
}

/// Nunca falla: si el archivo no existe o está corrupto, devuelve el
/// default vacío (Cordial usa sus propios valores por defecto en ese caso).
pub fn load(app: &AppHandle) -> LaunchEnv {
    let Ok(path) = settings_path(app) else {
        return LaunchEnv::default();
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        return LaunchEnv::default();
    };
    serde_json::from_str(&text).unwrap_or_default()
}

pub fn save(app: &AppHandle, env: &LaunchEnv) -> Result<(), String> {
    let path = settings_path(app)?;
    let text = serde_json::to_string_pretty(env)
        .map_err(|e| format!("no se pudo serializar la configuración de lanzamiento: {e}"))?;
    std::fs::write(&path, text).map_err(|e| format!("no se pudo guardar {}: {e}", path.display()))
}
