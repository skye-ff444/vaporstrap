//! Comandos IPC del módulo de optimización de hardware.

use tauri::AppHandle;

use super::probe::{self, HardwareSnapshot};
use super::recommend::{self, Recommendation};
use super::settings::{self, LaunchEnv};
use crate::fflags::document as fflags_document;

#[tauri::command]
pub fn hardware_probe() -> HardwareSnapshot {
    probe::snapshot()
}

#[tauri::command]
pub fn hardware_recommend(snapshot: HardwareSnapshot) -> Recommendation {
    recommend::recommend(&snapshot)
}

/// Qué variables de entorno están guardadas ahora mismo (para que la UI
/// pueda mostrar "ya tenés una optimización aplicada: throughput/automatic").
#[tauri::command]
pub fn hardware_current_launch_env(app: AppHandle) -> LaunchEnv {
    settings::load(&app)
}

/// Aplica una recomendación:
/// 1. Guarda `CORDIAL_PERFORMANCE`/`CORDIAL_GRAPHICS` para el próximo
///    lanzamiento de Cordial.
/// 2. Mezcla (sin reemplazar) el FPS cap y la calidad gráfica en el
///    `flags.json` del perfil de Cordial elegido, usando `merge_flags` para
///    no pisar un preset que el usuario ya haya aplicado desde el editor de
///    FastFlags.
#[tauri::command]
pub fn hardware_apply(
    app: AppHandle,
    profile: String,
    recommendation: Recommendation,
) -> Result<(), String> {
    let env = LaunchEnv {
        performance: Some(recommendation.performance.clone()),
        graphics: Some(recommendation.graphics.clone()),
    };
    settings::save(&app, &env)?;

    let updates = vec![
        (
            "DFIntTaskSchedulerTargetFps".to_string(),
            serde_json::Value::from(recommendation.fps_cap),
        ),
        (
            "FIntRenderGraphicsQualityOverride".to_string(),
            serde_json::Value::from(recommendation.graphics_quality),
        ),
    ];

    fflags_document::merge_flags(&profile, &updates)?;

    crate::logs::logger::info(
        &app,
        &format!(
            "Optimización de hardware aplicada al perfil '{profile}': performance={}, graphics={}, fps_cap={}, quality={}",
            recommendation.performance, recommendation.graphics, recommendation.fps_cap, recommendation.graphics_quality
        ),
    );

    Ok(())
}
