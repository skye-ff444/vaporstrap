//! Backend Cordial (<https://github.com/luohoa97/cordial>), GPL-3.0.
//!
//! Datos verificados contra el README/releases del proyecto (Cordial 0.9.0):
//! - No está en Flathub (política de Flathub sobre contenido asistido por
//!   IA), así que se distribuye con un remoto Flatpak propio.
//! - Desde la 0.8.0, Cordial descarga y verifica el build de Roblox por su
//!   cuenta (botón "Download Roblox" en su propia UI) — Vaporstrap no
//!   necesita gestionar el APK del usuario para este backend.
//! - Known issue documentado por el propio proyecto: el freeze de arranque
//!   no está resuelto a esta fecha. Se lo mostramos al usuario, no lo
//!   ocultamos.

use super::{flatpak, EngineStatus};

pub const REMOTE_NAME: &str = "cordial";
pub const REMOTE_URL: &str = "https://luohoa97.github.io/cordial/cordial.flatpakrepo";
pub const APP_ID: &str = "io.github.luohoa97.Cordial";
pub const KNOWN_ISSUES_URL: &str = "https://github.com/luohoa97/cordial/releases/latest";

/// Calcula el estado actual del motor consultando el sistema (no cachea).
pub fn status() -> EngineStatus {
    if !flatpak::is_flatpak_available() {
        return EngineStatus::FlatpakMissing;
    }
    if !flatpak::is_remote_added(REMOTE_NAME) {
        return EngineStatus::RemoteMissing;
    }
    match flatpak::installed_version(APP_ID) {
        Some(version) => EngineStatus::Installed { version },
        None => EngineStatus::NotInstalled,
    }
}

/// Agrega el remoto de Cordial (idempotente).
pub fn ensure_remote() -> Result<(), String> {
    flatpak::add_remote(REMOTE_NAME, REMOTE_URL)
}

/// Lanza Cordial vía `flatpak run` y devuelve el proceso hijo. No espera a
/// que termine acá mismo — quien llame decide si lo espera en un hilo aparte
/// (Vaporstrap lo hace siempre, aunque sea solo para evitar dejar un proceso
/// zombie sin reapear) o lo deja correr.
///
/// `env_vars` se pasa como `--env=CLAVE=VALOR` (soportado por `flatpak run`),
/// que es como el propio shell de Cordial fija sus variables
/// `CORDIAL_PERFORMANCE`/`CORDIAL_GRAPHICS` — tienen que estar seteadas
/// *antes* de que arranque el proceso, porque el motor las lee una sola vez
/// al iniciar.
///
/// `log_file`, si se da, redirige stdout y stderr del proceso a ese archivo
/// (ambos al mismo, para no perder el orden relativo). Sin esto, la salida
/// de Cordial se hereda de Vaporstrap y se pierde en cualquier lanzador
/// gráfico sin terminal — por eso el módulo `logs` siempre pasa una ruta.
pub fn launch(
    extra_args: &[String],
    env_vars: &[(String, String)],
    log_file: Option<&std::path::Path>,
) -> Result<std::process::Child, String> {
    let mut cmd = std::process::Command::new("flatpak");
    cmd.arg("run");

    for (key, value) in env_vars {
        cmd.arg(format!("--env={key}={value}"));
    }

    cmd.arg(APP_ID);
    cmd.args(extra_args);

    if let Some(path) = log_file {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("no se pudo crear {}: {e}", parent.display()))?;
        }
        let stdout_file = std::fs::File::create(path)
            .map_err(|e| format!("no se pudo crear el log {}: {e}", path.display()))?;
        let stderr_file = stdout_file
            .try_clone()
            .map_err(|e| format!("no se pudo preparar el log de Cordial: {e}"))?;
        cmd.stdout(std::process::Stdio::from(stdout_file));
        cmd.stderr(std::process::Stdio::from(stderr_file));
    }

    cmd.spawn().map_err(|e| format!("no se pudo lanzar Cordial: {e}"))
}
