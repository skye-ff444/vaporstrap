//! Wrapper delgado sobre el CLI de `flatpak`. No asume nada específico de
//! Cordial: cualquier backend futuro empaquetado como Flatpak puede
//! reutilizar estas funciones.

use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Arc;

/// True si el binario `flatpak` existe y responde.
pub fn is_flatpak_available() -> bool {
    Command::new("flatpak")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// True si el remoto `remote_name` ya está agregado (con cualquier URL).
pub fn is_remote_added(remote_name: &str) -> bool {
    let output = Command::new("flatpak")
        .args(["remotes", "--columns=name"])
        .output();

    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .lines()
            .any(|l| l.trim() == remote_name),
        _ => false,
    }
}

/// Agrega el remoto si no existe. Idempotente (`--if-not-exists`).
pub fn add_remote(remote_name: &str, remote_url: &str) -> Result<(), String> {
    let output = Command::new("flatpak")
        .args(["remote-add", "--if-not-exists", remote_name, remote_url])
        .output()
        .map_err(|e| format!("no se pudo ejecutar flatpak: {e}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

/// Devuelve la versión instalada de `app_id`, o `None` si no está instalado.
pub fn installed_version(app_id: &str) -> Option<String> {
    let output = Command::new("flatpak")
        .args(["list", "--app", "--columns=application,version"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| {
            let mut parts = line.split('\t');
            let id = parts.next()?.trim();
            let version = parts.next().unwrap_or("").trim();
            (id == app_id).then(|| version.to_string())
        })
}

/// Corre `flatpak install -y <remote> <app_id>`, entregando cada línea de
/// stdout/stderr a `on_line` a medida que llega (para mostrar progreso en
/// la UI). Bloqueante: se espera que quien llame lo haga desde un hilo
/// dedicado, no desde el hilo principal de la UI.
pub fn install_app_streaming(
    remote_name: &str,
    app_id: &str,
    on_line: impl Fn(String) + Send + Sync + 'static,
) -> Result<ExitStatus, String> {
    let mut child = Command::new("flatpak")
        .args(["install", "-y", remote_name, app_id])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("no se pudo ejecutar flatpak install: {e}"))?;

    let on_line = Arc::new(on_line);

    let stdout = child.stdout.take().expect("stdout debería estar piped");
    let stderr = child.stderr.take().expect("stderr debería estar piped");

    let cb_out = Arc::clone(&on_line);
    let stdout_thread = std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            cb_out(line);
        }
    });

    let cb_err = Arc::clone(&on_line);
    let stderr_thread = std::thread::spawn(move || {
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            cb_err(line);
        }
    });

    let status = child
        .wait()
        .map_err(|e| format!("error esperando a flatpak install: {e}"))?;

    let _ = stdout_thread.join();
    let _ = stderr_thread.join();

    Ok(status)
}
