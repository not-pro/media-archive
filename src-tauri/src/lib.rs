pub mod domain;
pub mod infrastructure;
mod platform;
pub mod presentation;

use tauri::Manager;

use platform::directory_commands::{directory_choose, directory_open};
use platform::menu::fixed_window_menu;
use platform::session_commands::{session_clear, session_import, session_status};
use platform::sidecars::{sidecar_paths, verify_sidecars};
use platform::state::AppState;
use platform::video_commands::{download_cancel, download_start, video_inspect};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.set_menu(fixed_window_menu(app.handle())?)?;
            let paths = sidecar_paths(app.handle())?;
            verify_sidecars(&paths)?;
            app.manage(AppState::new(paths));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            session_status,
            session_import,
            session_clear,
            video_inspect,
            directory_choose,
            directory_open,
            download_start,
            download_cancel,
        ])
        .run(tauri::generate_context!())
        .expect("Media Archive failed to start");
}
