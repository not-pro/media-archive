use std::path::Path;

use serde_json::{Value, json};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

use crate::domain::error::AppError;
use crate::platform::state::AppState;
use crate::presentation::view::{PublicError, present_error};

#[tauri::command]
pub fn directory_choose(app: AppHandle, state: State<'_, AppState>) -> Result<Value, PublicError> {
    let selected = app
        .dialog()
        .file()
        .blocking_pick_folder()
        .map(|path| path.into_path())
        .transpose()
        .map_err(|error| present_error(AppError::new("DIRECTORY_INVALID", error.to_string())))?;
    if let Some(directory) = selected {
        *state.directory.lock().unwrap() = Some(directory);
    }
    let directory = state.directory.lock().unwrap().clone();
    Ok(json!({ "directory": directory.map(|path| path.display().to_string()) }))
}

#[tauri::command]
pub fn directory_open(app: AppHandle, directory: String) -> Result<Value, PublicError> {
    if !Path::new(&directory).is_dir() {
        return Err(present_error(AppError::new(
            "DIRECTORY_INVALID",
            "Папка недоступна.",
        )));
    }
    app.opener()
        .open_path(directory, None::<&str>)
        .map_err(|error| {
            present_error(AppError::new("DIRECTORY_OPEN_FAILED", error.to_string()))
        })?;
    Ok(json!({ "opened": true }))
}
