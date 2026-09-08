//! Conecta el ciclo de vida del proceso de Cordial con la presencia de
//! Discord — y, de paso, resuelve un problema que ya existía desde el punto
//! 2 del MVP: `engine::cordial::launch` devuelve un `Child` que si nadie
//! espera (`wait()`), queda como proceso zombie hasta que Vaporstrap mismo
//! cierre. Este hilo lo espera siempre, tenga Discord habilitado o no.

use std::process::Child;

use tauri::AppHandle;

use super::{client, settings};

/// Se llama una vez por cada `Child` que devuelve `engine::cordial::launch`.
/// Siempre reapea el proceso; además, si Discord RPC está habilitado y
/// configurado con un Client ID, marca la actividad al empezar y la limpia
/// al terminar.
pub fn watch_process(app: AppHandle, mut child: Child) {
    std::thread::spawn(move || {
        let discord_settings = settings::load(&app);
        let client_id = discord_settings
            .enabled
            .then(|| discord_settings.client_id.clone())
            .flatten()
            .filter(|id| !id.trim().is_empty());

        if let Some(id) = &client_id {
            let start = client::now_unix_secs();
            if let Err(e) = client::set_activity(id, "Jugando vía Cordial", "En Linux, vía Vaporstrap", start) {
                eprintln!("[discord] no se pudo activar la presencia: {e}");
            }
        }

        // Pase lo que pase con Discord, siempre esperamos al proceso para
        // que el sistema operativo pueda liberar sus recursos.
        let _ = child.wait();

        if let Some(id) = &client_id {
            if let Err(e) = client::clear_activity(id) {
                eprintln!("[discord] no se pudo limpiar la presencia: {e}");
            }
        }
    });
}
