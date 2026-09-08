//! Protocolo IPC de Discord, implementado a mano sobre `UnixStream` de la
//! librería estándar — sin dependencias nuevas, ya que `serde_json` ya es
//! una dependencia del proyecto.
//!
//! Formato de cada mensaje (documentado de forma no oficial, pero estable y
//! usado por prácticamente todas las integraciones de Rich Presence):
//! 4 bytes little-endian con el opcode, 4 bytes little-endian con el largo
//! del cuerpo, y el cuerpo en JSON.

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::Duration;

pub const OPCODE_HANDSHAKE: u32 = 0;
pub const OPCODE_FRAME: u32 = 1;

/// Directorios donde puede vivir el socket `discord-ipc-<n>`. Incluye la
/// ruta anidada que usa la build Flatpak de Discord y la que usa la build
/// snap (común en distros basadas en Ubuntu, como la que corre este
/// proyecto en desarrollo).
fn candidate_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        let base = PathBuf::from(&runtime);
        dirs.push(base.clone());
        dirs.push(base.join("app/com.discordapp.Discord")); // Discord Flatpak
        dirs.push(base.join("snap.discord")); // Discord snap
    }

    // Algunos entornos exponen el socket acá si XDG_RUNTIME_DIR no está seteada.
    dirs.push(PathBuf::from("/tmp"));

    dirs
}

/// Prueba `discord-ipc-0` a `discord-ipc-9` en cada directorio candidato.
/// `None` si Discord no está corriendo o no se encontró el socket.
pub fn connect() -> Option<UnixStream> {
    for dir in candidate_dirs() {
        for i in 0..10 {
            let path = dir.join(format!("discord-ipc-{i}"));
            if let Ok(stream) = UnixStream::connect(&path) {
                let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
                let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
                return Some(stream);
            }
        }
    }
    None
}

pub fn write_frame(
    stream: &mut UnixStream,
    opcode: u32,
    payload: &serde_json::Value,
) -> std::io::Result<()> {
    let body = serde_json::to_vec(payload)?;
    stream.write_all(&opcode.to_le_bytes())?;
    stream.write_all(&(body.len() as u32).to_le_bytes())?;
    stream.write_all(&body)?;
    stream.flush()
}

/// Lee un frame de respuesta. Con timeout corto (ver `connect`): si Discord
/// no contesta a tiempo, devuelve un error en vez de bloquear para siempre.
pub fn read_frame(stream: &mut UnixStream) -> std::io::Result<(u32, serde_json::Value)> {
    let mut header = [0u8; 8];
    stream.read_exact(&mut header)?;

    let opcode = u32::from_le_bytes(header[0..4].try_into().unwrap());
    let len = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;

    let mut body = vec![0u8; len];
    stream.read_exact(&mut body)?;

    let value = serde_json::from_slice(&body).unwrap_or(serde_json::Value::Null);
    Ok((opcode, value))
}
