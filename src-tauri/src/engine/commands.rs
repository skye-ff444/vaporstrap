//! Comandos IPC de Tauri para el frontend. Mantiene la lógica de sistema en
//! `flatpak`/`cordial`; este archivo solo la adapta al mundo de Tauri
//! (comandos + eventos).

use tauri::{AppHandle, Emitter};

use super::{cordial, EngineStatus};

/// Devuelve el estado actual del motor (sin efectos secundarios).
#[tauri::command]
pub fn engine_status() -> EngineStatus {
    cordial::status()
}

/// URL de la página de "known issues" del backend activo, para que la UI
/// pueda enlazarla sin hardcodearla en el frontend.
#[tauri::command]
pub fn engine_known_issues_url() -> &'static str {
    cordial::KNOWN_ISSUES_URL
}

/// Dispara la instalación del motor en un hilo aparte y devuelve de
/// inmediato. El progreso se comunica vía eventos:
///   - "engine://install-log"   (String)  una línea de salida de flatpak
///   - "engine://install-done"  (bool)    true si terminó con éxito
///   - "engine://install-error" (String)  mensaje de error
#[tauri::command]
pub fn engine_install(app: AppHandle) -> Result<(), String> {
    if matches!(cordial::status(), EngineStatus::RemoteMissing) {
        cordial::ensure_remote()?;
        let _ = app.emit(
            "engine://install-log",
            "Remoto de Cordial agregado.".to_string(),
        );
    }

    crate::logs::logger::info(&app, "Instalación de Cordial iniciada.");

    std::thread::spawn(move || {
        let app_for_lines = app.clone();
        let result = super::flatpak::install_app_streaming(
            cordial::REMOTE_NAME,
            cordial::APP_ID,
            move |line| {
                let _ = app_for_lines.emit("engine://install-log", line);
            },
        );

        match result {
            Ok(status) if status.success() => {
                crate::logs::logger::info(&app, "Cordial instalado correctamente.");
                let _ = app.emit("engine://install-done", true);
            }
            Ok(status) => {
                let msg = format!("flatpak install terminó con código {:?}", status.code());
                crate::logs::logger::error(&app, &msg);
                let _ = app.emit("engine://install-error", msg);
            }
            Err(e) => {
                crate::logs::logger::error(&app, &format!("Error instalando Cordial: {e}"));
                let _ = app.emit("engine://install-error", e);
            }
        }
    });

    Ok(())
}

/// Lanza Cordial. Falla rápido si el proceso no pudo ni siquiera arrancar;
/// no espera a que el usuario cierre Roblox.
///
/// Usa las variables de entorno guardadas por el módulo de optimización de
/// hardware (punto 4 del MVP), si el usuario ya aplicó una recomendación.
/// Si nunca lo hizo, `env.to_pairs()` está vacío y Cordial usa sus propios
/// valores por defecto (balanced / automatic).
///
/// Redirige la salida del proceso a un archivo de log nuevo (punto 6 del
/// MVP) y entrega el proceso a `discord::presence::watch_process`, que lo
/// espera en un hilo aparte (evita dejarlo zombie) y, si Discord RPC está
/// habilitado, marca/limpia la actividad alrededor de su ciclo de vida.
#[tauri::command]
pub fn engine_launch(app: AppHandle) -> Result<(), String> {
    let env = crate::hardware::settings::load(&app);
    let log_path = crate::logs::paths::new_cordial_log_path(&app).ok();

    let child = cordial::launch(&[], &env.to_pairs(), log_path.as_deref())?;
    crate::logs::logger::info(&app, "Cordial lanzado desde el botón 'Iniciar Cordial'.");

    crate::discord::presence::watch_process(app, child);
    Ok(())
}
