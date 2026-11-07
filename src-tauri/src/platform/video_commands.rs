use std::path::PathBuf;

use serde_json::{Value, json};
use tauri::{AppHandle, Emitter, State};

use crate::domain::entities::DownloadOptions;
use crate::domain::error::{AppError, AppResult};
use crate::platform::state::AppState;
use crate::platform::tasks::blocking;
use crate::presentation::view::{PublicError, present_error};

#[tauri::command]
pub async fn video_inspect(state: State<'_, AppState>, url: String) -> Result<Value, PublicError> {
    let videos = state.videos.clone();
    let sessions = state.sessions.clone();
    blocking(move || {
        let metadata =
            sessions.with_cookie_file_or_none(|cookies| videos.inspect(&url, cookies))?;
        Ok::<Value, AppError>(json!(metadata))
    })
    .await
}

fn validate_directory(options: &DownloadOptions) -> AppResult<PathBuf> {
    let directory = PathBuf::from(&options.output_directory);
    if !directory.is_absolute() || !directory.is_dir() {
        return Err(AppError::new(
            "DIRECTORY_INVALID",
            "Выберите папку сохранения.",
        ));
    }
    Ok(directory)
}

#[tauri::command]
pub async fn download_start(
    app: AppHandle,
    state: State<'_, AppState>,
    options: DownloadOptions,
) -> Result<Value, PublicError> {
    let directory = validate_directory(&options).map_err(present_error)?;
    *state.directory.lock().unwrap() = Some(directory);
    let videos = state.videos.clone();
    let sessions = state.sessions.clone();
    blocking(move || {
        let result = sessions.with_cookie_file_or_none(|cookies| {
            videos.download(&options, cookies, &|progress| {
                let _ = app.emit("download:progress", progress);
            })
        })?;
        Ok::<Value, AppError>(json!(result))
    })
    .await
}

#[tauri::command]
pub fn download_cancel(state: State<'_, AppState>) -> Value {
    json!({ "cancelled": state.videos.cancel() })
}
