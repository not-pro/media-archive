use std::fs;
use std::path::Path;

use keyring::{Entry, Error as KeyringError};
use tempfile::tempdir;

use crate::application::ports::SessionGateway;
use crate::domain::error::{AppError, AppResult};
use crate::infrastructure::session::filter::filter_session_cookies;

#[derive(Default)]
pub struct KeychainSessionStore;

impl KeychainSessionStore {
    pub fn new() -> Self {
        Self
    }

    fn entry(&self) -> AppResult<Entry> {
        Entry::new("local.mediaarchive", "session")
            .map_err(|error| AppError::new("SESSION_STORE_ERROR", error.to_string()))
    }

    fn secret(&self) -> AppResult<String> {
        self.entry()?.get_password().map_err(|error| {
            AppError::new(
                "COOKIES_INVALID",
                format!("Сессию не удалось прочитать: {error}"),
            )
        })
    }
}

impl SessionGateway for KeychainSessionStore {
    fn has_session(&self) -> AppResult<bool> {
        match self.entry()?.get_password() {
            Ok(_) => Ok(true),
            Err(KeyringError::NoEntry) => Ok(false),
            Err(error) => Err(AppError::new("SESSION_STORE_ERROR", error.to_string())),
        }
    }

    fn import_file(&self, source: &Path) -> AppResult<()> {
        let metadata = fs::metadata(source)
            .map_err(|error| AppError::new("COOKIES_INVALID", error.to_string()))?;
        if !metadata.is_file() || metadata.len() > 5 * 1024 * 1024 {
            return Err(AppError::new(
                "COOKIES_INVALID",
                "Файл слишком большой или недоступен.",
            ));
        }
        let contents = fs::read_to_string(source)
            .map_err(|error| AppError::new("COOKIES_INVALID", error.to_string()))?;
        let filtered = filter_session_cookies(&contents)?;
        self.entry()?
            .set_password(&filtered)
            .map_err(|error| AppError::new("SESSION_STORE_ERROR", error.to_string()))
    }

    fn clear(&self) -> AppResult<()> {
        match self.entry()?.delete_credential() {
            Ok(()) | Err(KeyringError::NoEntry) => Ok(()),
            Err(error) => Err(AppError::new("SESSION_STORE_ERROR", error.to_string())),
        }
    }

    fn with_cookie_file<T>(&self, operation: impl FnOnce(&Path) -> AppResult<T>) -> AppResult<T> {
        let directory =
            tempdir().map_err(|error| AppError::new("SESSION_IO_ERROR", error.to_string()))?;
        let file = directory.path().join("cookies.txt");
        write_private(&file, self.secret()?.as_bytes())?;
        let result = operation(&file)?;
        let updated = fs::read_to_string(&file)
            .map_err(|error| AppError::new("SESSION_IO_ERROR", error.to_string()))?;
        self.entry()?
            .set_password(&filter_session_cookies(&updated)?)
            .map_err(|error| AppError::new("SESSION_STORE_ERROR", error.to_string()))?;
        Ok(result)
    }
}

#[cfg(unix)]
fn write_private(file: &Path, contents: &[u8]) -> AppResult<()> {
    use std::os::unix::fs::OpenOptionsExt;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(file)
        .and_then(|mut file| std::io::Write::write_all(&mut file, contents))
        .map_err(|error| AppError::new("SESSION_IO_ERROR", error.to_string()))
}

#[cfg(windows)]
fn write_private(file: &Path, contents: &[u8]) -> AppResult<()> {
    fs::write(file, contents).map_err(|error| AppError::new("SESSION_IO_ERROR", error.to_string()))
}
