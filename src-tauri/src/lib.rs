use tauri::Manager;

mod commands;
mod foss_db;
mod registry;

pub use commands::*;

use window_vibrancy::apply_mica;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Apply Windows 11 Mica effect
            #[cfg(target_os = "windows")]
            {
                apply_mica(&window, Some(true)).expect("Failed to apply Mica effect");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_installed_software,
            commands::get_foss_alternatives,
            commands::uninstall_software,
            commands::get_all_foss_apps,
            commands::download_foss_app,
            commands::check_winget_available,
            commands::search_winget,
            commands::install_winget,
            commands::fetch_winget_api,
            commands::search_winget_api
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
