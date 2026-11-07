use std::path::Path;

use crate::domain::entities::{DownloadOptions, DownloadProgress, DownloadResult, VideoMetadata};
use crate::domain::error::AppResult;

pub trait VideoGateway: Send + Sync {
    fn inspect(&self, url: &str, cookies: Option<&Path>) -> AppResult<VideoMetadata>;
    fn download(
        &self,
        options: &DownloadOptions,
        cookies: Option<&Path>,
        progress: &dyn Fn(DownloadProgress),
    ) -> AppResult<DownloadResult>;
    fn cancel(&self) -> bool;
}

pub trait SessionGateway: Send + Sync {
    fn has_session(&self) -> AppResult<bool>;
    fn import_file(&self, source: &Path) -> AppResult<()>;
    fn clear(&self) -> AppResult<()>;
    fn with_cookie_file<T>(&self, operation: impl FnOnce(&Path) -> AppResult<T>) -> AppResult<T>;
}
