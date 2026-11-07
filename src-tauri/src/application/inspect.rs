use std::sync::Arc;

use crate::application::ports::{SessionGateway, VideoGateway};
use crate::application::session::with_optional_cookies;
use crate::domain::entities::VideoMetadata;
use crate::domain::error::AppResult;

pub struct InspectVideoInteractor<V, S> {
    videos: Arc<V>,
    sessions: Arc<S>,
}

impl<V: VideoGateway, S: SessionGateway> InspectVideoInteractor<V, S> {
    pub fn new(videos: Arc<V>, sessions: Arc<S>) -> Self {
        Self { videos, sessions }
    }

    pub fn execute(&self, url: &str) -> AppResult<VideoMetadata> {
        with_optional_cookies(self.sessions.as_ref(), |cookies| {
            self.videos.inspect(url, cookies)
        })
    }
}
