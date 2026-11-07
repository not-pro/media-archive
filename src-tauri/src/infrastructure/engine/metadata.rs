use std::collections::BTreeSet;

use serde_json::Value;

use crate::domain::entities::{VideoMetadata, VideoQuality};
use crate::domain::error::{AppError, AppResult};

pub fn parse_metadata(raw: &str) -> AppResult<VideoMetadata> {
    let data: Value = serde_json::from_str(raw).map_err(|_| {
        AppError::new(
            "METADATA_INVALID",
            "Загрузчик вернул некорректные метаданные.",
        )
    })?;
    if data["_type"] == "playlist" || data["is_live"] == true || data["live_status"] == "is_live" {
        return Err(AppError::new(
            "VIDEO_UNSUPPORTED",
            "Плейлисты и текущие прямые трансляции не поддерживаются.",
        ));
    }
    let id = data["id"]
        .as_str()
        .ok_or_else(|| AppError::new("METADATA_INVALID", "В ответе нет идентификатора видео."))?;
    let title = data["title"]
        .as_str()
        .ok_or_else(|| AppError::new("METADATA_INVALID", "В ответе нет названия видео."))?;
    let mut heights = BTreeSet::new();
    for format in data["formats"].as_array().into_iter().flatten() {
        let height = format["height"].as_u64().unwrap_or(0) as u32;
        if format["vcodec"] != "none" && (144..=4320).contains(&height) {
            heights.insert(height);
        }
    }
    Ok(VideoMetadata {
        id: id.into(),
        title: title.into(),
        uploader: data["uploader"].as_str().unwrap_or_default().into(),
        duration: data["duration"].as_f64(),
        thumbnail: data["thumbnail"]
            .as_str()
            .filter(|url| url.starts_with("https://"))
            .map(Into::into),
        qualities: heights
            .into_iter()
            .rev()
            .map(|height| VideoQuality {
                height,
                label: format!("{height}p"),
            })
            .collect(),
    })
}
