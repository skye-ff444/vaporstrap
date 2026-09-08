// Comandos expuestos al frontend vía IPC de Tauri.
// A medida que avance el MVP, esto se irá dividiendo en más módulos:
//   - engine::   integración con el motor (Cordial)   [MVP punto 2]
//   - fflags::   editor de FastFlags                  [MVP punto 3]
//   - profile::  perfiles de optimización por hardware [MVP punto 4]

mod discord;
mod engine;
mod fflags;
mod hardware;
mod logs;
mod profiles;

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_app_version,
            engine::commands::engine_status,
            engine::commands::engine_known_issues_url,
            engine::commands::engine_install,
            engine::commands::engine_launch,
            fflags::commands::fflags_catalog,
            fflags::commands::fflags_list_presets,
            fflags::commands::fflags_load_preset,
            fflags::commands::fflags_save_preset,
            fflags::commands::fflags_delete_preset,
            fflags::commands::fflags_import_json,
            fflags::commands::fflags_export_json,
            fflags::commands::fflags_list_cordial_profiles,
            fflags::commands::fflags_read_cordial_profile,
            fflags::commands::fflags_apply_to_cordial,
            hardware::commands::hardware_probe,
            hardware::commands::hardware_recommend,
            hardware::commands::hardware_current_launch_env,
            hardware::commands::hardware_apply,
            discord::commands::discord_get_settings,
            discord::commands::discord_save_settings,
            discord::commands::discord_test,
            discord::commands::discord_clear,
            profiles::commands::profiles_list,
            profiles::commands::profiles_create,
            profiles::commands::profiles_delete,
            profiles::commands::profiles_rename,
            profiles::commands::profiles_launch_with_link,
            logs::commands::logs_list_cordial_launches,
            logs::commands::logs_read,
            logs::commands::logs_read_vaporstrap,
            logs::commands::logs_dir_path,
            logs::commands::logs_generate_diagnostic_report,
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Vaporstrap");
}
