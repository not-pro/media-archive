use std::sync::Arc;

use crate::application::ports::{SessionGateway, VideoGateway};
use crate::application::session::with_optional_cookies;
use crate::domain::entities::{DownloadOptions, DownloadProgress, DownloadResult};
use crate::domain::error::AppResult;

pub struct DownloadVideoInteractor<V, S> {
    videos: Arc<V>,
    sessions: Arc<S>,
}

impl<V: VideoGateway, S: SessionGateway> DownloadVideoInteractor<V, S> {
    pub fn new(videos: Arc<V>, sessions: Arc<S>) -> Self {
        Self { videos, sessions }
    }

    pub fn execute(
        &self,
        options: &DownloadOptions,
        progress: &dyn Fn(DownloadProgress),
    ) -> AppResult<DownloadResult> {
        with_optional_cookies(self.sessions.as_ref(), |cookies| {
            self.videos.download(options, cookies, progress)
        })
    }

    pub fn cancel(&self) -> bool {
        self.videos.cancel()
    }
}
