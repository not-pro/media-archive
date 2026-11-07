use crate::domain::error::{AppError, AppResult};
use crate::presentation::view::{PublicError, present_error};

pub async fn blocking<T, F>(task: F) -> Result<T, PublicError>
where
    T: Send + 'static,
    F: FnOnce() -> AppResult<T> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(task)
        .await
        .map_err(|error| present_error(AppError::new("TASK_FAILED", error.to_string())))?
        .map_err(present_error)
}
