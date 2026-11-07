use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::application::download::DownloadVideoInteractor;
use crate::application::inspect::InspectVideoInteractor;
use crate::application::session::SessionInteractor;
use crate::infrastructure::engine::args::SidecarPaths;
use crate::infrastructure::engine::gateway::MediaGateway;
use crate::infrastructure::session::store::KeychainSessionStore;

#[derive(Clone)]
pub struct AppState {
    pub inspect: Arc<InspectVideoInteractor<MediaGateway, KeychainSessionStore>>,
    pub downloads: Arc<DownloadVideoInteractor<MediaGateway, KeychainSessionStore>>,
    pub sessions: Arc<SessionInteractor<KeychainSessionStore>>,
    pub directory: Arc<Mutex<Option<PathBuf>>>,
}

impl AppState {
    pub fn new(paths: SidecarPaths) -> Self {
        let videos = Arc::new(MediaGateway::new(paths));
        let sessions = Arc::new(KeychainSessionStore::new());
        Self {
            inspect: Arc::new(InspectVideoInteractor::new(
                videos.clone(),
                sessions.clone(),
            )),
            downloads: Arc::new(DownloadVideoInteractor::new(videos, sessions.clone())),
            sessions: Arc::new(SessionInteractor::new(sessions)),
            directory: Arc::new(Mutex::new(None)),
        }
    }
}
