use url::Url;

use crate::domain::error::{AppError, AppResult};

const HOSTS: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "m.youtube.com",
    "music.youtube.com",
    "youtube-nocookie.com",
    "www.youtube-nocookie.com",
    "youtu.be",
    "www.youtu.be",
];

fn invalid() -> AppError {
    AppError::new("URL_INVALID", "Введите корректную HTTPS-ссылку на видео.")
}

fn valid_id(id: &str) -> bool {
    id.len() == 11
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn video_id(url: &Url, host: &str) -> Option<String> {
    let segments: Vec<&str> = url.path_segments()?.filter(|s| !s.is_empty()).collect();
    if host.ends_with("youtu.be") {
        return segments.first().map(|id| id.to_string());
    }
    if let Some(id) = url
        .query_pairs()
        .find(|(key, _)| key == "v")
        .map(|(_, id)| id)
    {
        return Some(id.into_owned());
    }
    if matches!(segments.first(), Some(&"shorts" | &"live" | &"embed")) {
        return segments.get(1).map(|id| id.to_string());
    }
    None
}

pub fn normalize_media_url(raw: &str) -> AppResult<String> {
    let url = Url::parse(raw.trim()).map_err(|_| invalid())?;
    let host = url.host_str().unwrap_or_default().to_lowercase();
    if url.scheme() != "https" || !HOSTS.contains(&host.as_str()) {
        return Err(invalid());
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(invalid());
    }
    let id = video_id(&url, &host)
        .filter(|id| valid_id(id))
        .ok_or_else(invalid)?;
    Ok(format!("https://www.youtube.com/watch?v={id}"))
}
