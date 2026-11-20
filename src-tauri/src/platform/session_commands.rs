use serde_json::{Value, json};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use crate::domain::error::AppError;
use crate::platform::state::AppState;
use crate::platform::tasks::blocking;
use crate::presentation::view::{PublicError, present_error};

#[tauri::command]
pub async fn session_status(state: State<'_, AppState>) -> Result<Value, PublicError> {
    let sessions = state.sessions.clone();
    blocking(move || {
        sessions
            .has_session()
            .map(|connected| json!({ "connected": connected }))
    })
    .await
}

#[tauri::command]
pub async fn session_import(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Value, PublicError> {
    let source = app
        .dialog()
        .file()
        .add_filter("Netscape cookies", &["txt"])
        .blocking_pick_file();
    let sessions = state.sessions.clone();
    let Some(source) = source else {
        return blocking(move || {
            sessions
                .has_session()
                .map(|connected| json!({ "connected": connected }))
        })
        .await;
    };
    let source = source
        .into_path()
        .map_err(|error| present_error(AppError::new("COOKIES_INVALID", error.to_string())))?;
    blocking(move || {
        sessions.import_file(&source)?;
        Ok(json!({ "connected": true }))
    })
    .await
}

#[tauri::command]
pub async fn session_clear(state: State<'_, AppState>) -> Result<Value, PublicError> {
    state.videos.cancel();
    let sessions = state.sessions.clone();
    blocking(move || {
        sessions.clear()?;
        Ok(json!({ "connected": false }))
    })
    .await
}
