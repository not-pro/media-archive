use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::infrastructure::engine::args::SidecarPaths;
use crate::infrastructure::engine::gateway::MediaGateway;
use crate::infrastructure::session::store::KeychainSessionStore;

#[derive(Clone)]
pub struct AppState {
    pub videos: Arc<MediaGateway>,
    pub sessions: Arc<KeychainSessionStore>,
    pub directory: Arc<Mutex<Option<PathBuf>>>,
}

impl AppState {
    pub fn new(paths: SidecarPaths) -> Self {
        Self {
            videos: Arc::new(MediaGateway::new(paths)),
            sessions: Arc::new(KeychainSessionStore::new()),
            directory: Arc::new(Mutex::new(None)),
        }
    }
}
