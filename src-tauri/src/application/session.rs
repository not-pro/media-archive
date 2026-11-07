use std::path::Path;
use std::sync::Arc;

use crate::application::ports::SessionGateway;
use crate::domain::error::AppResult;

pub fn with_optional_cookies<S, T>(
    sessions: &S,
    operation: impl FnOnce(Option<&Path>) -> AppResult<T>,
) -> AppResult<T>
where
    S: SessionGateway,
{
    if sessions.has_session()? {
        sessions.with_cookie_file(|file| operation(Some(file)))
    } else {
        operation(None)
    }
}

pub struct SessionInteractor<S> {
    sessions: Arc<S>,
}

impl<S: SessionGateway> SessionInteractor<S> {
    pub fn new(sessions: Arc<S>) -> Self {
        Self { sessions }
    }

    pub fn status(&self) -> AppResult<bool> {
        self.sessions.has_session()
    }

    pub fn import(&self, source: &Path) -> AppResult<()> {
        self.sessions.import_file(source)
    }

    pub fn clear(&self) -> AppResult<()> {
        self.sessions.clear()
    }
}
