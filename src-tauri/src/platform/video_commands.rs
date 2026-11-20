use std::path::PathBuf;

use serde_json::{Value, json};
use tauri::{AppHandle, Emitter, State};

use crate::domain::entities::DownloadOptions;
use crate::domain::error::{AppError, AppResult};
use crate::infrastructure::engine::args::Cookies;
use crate::infrastructure::session::browser::available_browsers;
use crate::infrastructure::session::store::KeychainSessionStore;
use crate::platform::state::AppState;
use crate::platform::tasks::blocking;
use crate::presentation::view::{PublicError, present_error};

fn needs_session(error: &AppError) -> bool {
    matches!(
        error.code,
        "ACCESS_DENIED" | "AUTH_REQUIRED" | "AGE_RESTRICTED" | "MEMBERS_ONLY"
    )
}

fn with_session<T>(
    sessions: &KeychainSessionStore,
    browsers: impl IntoIterator<Item = &'static str>,
    mut operation: impl FnMut(Option<Cookies<'_>>) -> AppResult<T>,
) -> AppResult<T> {
    let anonymous_error = match operation(None) {
        Err(error) if needs_session(&error) => error,
        result => return result,
    };
    for browser in browsers {
        match operation(Some(Cookies::Browser(browser))) {
            Err(error) if needs_session(&error) || error.code == "BROWSER_SESSION_UNAVAILABLE" => {}
            result => return result,
        }
    }
    if sessions.has_session()? {
        return sessions.with_cookie_file(|file| operation(Some(Cookies::File(file))));
    }
    Err(anonymous_error)
}

#[tauri::command]
pub async fn video_inspect(state: State<'_, AppState>, url: String) -> Result<Value, PublicError> {
    let videos = state.videos.clone();
    let sessions = state.sessions.clone();
    blocking(move || {
        let metadata = with_session(&sessions, available_browsers(), |cookies| {
            videos.inspect(&url, cookies)
        })?;
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
        let result = with_session(&sessions, available_browsers(), |cookies| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::engine::error_mapper::map_engine_error;

    #[test]
    fn private_video_tries_the_next_browser_if_the_first_is_unavailable() {
        let mut attempts = 0;
        let result = with_session(
            &KeychainSessionStore::new(),
            ["chrome", "firefox"],
            |cookies| {
                attempts += 1;
                match cookies {
                    None => Err(map_engine_error(
                        "Private video. Sign in if you've been granted access.",
                    )),
                    Some(Cookies::Browser("chrome")) => {
                        Err(map_engine_error("Could not copy Chrome cookie database"))
                    }
                    Some(Cookies::Browser("firefox")) => Ok("accessible"),
                    _ => panic!("unexpected session source"),
                }
            },
        );
        assert_eq!(result.unwrap(), "accessible");
        assert_eq!(attempts, 3);
    }
}
