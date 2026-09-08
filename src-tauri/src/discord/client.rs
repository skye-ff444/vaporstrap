//! API de alto nivel sobre `ipc`: handshake + `SET_ACTIVITY`.
//!
//! Cada llamada abre su propia conexión y la cierra al terminar — no
//! mantenemos un socket persistente. Para el uso que le da Vaporspot
//! (una activación al lanzar Cordial, una limpieza al cerrarse) es más
//! simple y no hay ganancia real en mantener la conexión viva.

use serde_json::json;

use super::ipc::{self, OPCODE_FRAME, OPCODE_HANDSHAKE};

fn handshake(stream: &mut std::os::unix::net::UnixStream, client_id: &str) -> Result<(), String> {
    ipc::write_frame(stream, OPCODE_HANDSHAKE, &json!({ "v": 1, "client_id": client_id }))
        .map_err(|e| format!("no se pudo enviar el handshake a Discord: {e}"))?;

    // Best-effort: intentamos leer la respuesta (READY/ERROR) pero no
    // fallamos la operación si Discord tarda más que el timeout del socket.
    let _ = ipc::read_frame(stream);
    Ok(())
}

/// Discord corta `details`/`state` en 128 caracteres y rechaza el payload
/// entero si se los mandás más largos — mejor cortarlo nosotros y que se
/// vea truncado a que la actividad no aparezca en absoluto.
fn truncate_128(s: &str) -> String {
    s.chars().take(128).collect()
}

pub fn now_unix_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn nonce() -> String {
    // Un nonce único por request es lo único que exige el protocolo; no hace
    // falta que sea criptográficamente aleatorio para este uso.
    format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0))
}

pub fn set_activity(client_id: &str, details: &str, state: &str, start_unix_secs: i64) -> Result<(), String> {
    let mut stream = ipc::connect()
        .ok_or_else(|| "no se encontró el socket IPC de Discord (¿está Discord abierto?)".to_string())?;
    handshake(&mut stream, client_id)?;

    let payload = json!({
        "cmd": "SET_ACTIVITY",
        "args": {
            "pid": std::process::id(),
            "activity": {
                "details": truncate_128(details),
                "state": truncate_128(state),
                "timestamps": { "start": start_unix_secs },
                "assets": {
                    // Requiere subir una imagen con esta clave en la sección
                    // "Rich Presence Assets" de la aplicación de Discord del
                    // client_id configurado. Si no existe, Discord
                    // simplemente no muestra imagen — no falla el resto.
                    "large_image": "vaporstrap-logo",
                    "large_text": "Vaporstrap"
                }
            }
        },
        "nonce": nonce(),
    });

    ipc::write_frame(&mut stream, OPCODE_FRAME, &payload)
        .map_err(|e| format!("no se pudo enviar la actividad a Discord: {e}"))
}

pub fn clear_activity(client_id: &str) -> Result<(), String> {
    let mut stream = ipc::connect()
        .ok_or_else(|| "no se encontró el socket IPC de Discord".to_string())?;
    handshake(&mut stream, client_id)?;

    let payload = json!({
        "cmd": "SET_ACTIVITY",
        "args": { "pid": std::process::id(), "activity": null },
        "nonce": nonce(),
    });

    ipc::write_frame(&mut stream, OPCODE_FRAME, &payload)
        .map_err(|e| format!("no se pudo limpiar la actividad de Discord: {e}"))
}
