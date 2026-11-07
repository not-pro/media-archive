use serde::Serialize;

use crate::domain::entities::VideoMetadata;
use crate::domain::error::AppError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoViewModel {
    #[serde(flatten)]
    pub metadata: VideoMetadata,
    pub duration_label: String,
}

fn duration_label(seconds: Option<f64>) -> String {
    let Some(seconds) = seconds else {
        return String::new();
    };
    let whole = seconds.max(0.0).round() as u64;
    let hours = whole / 3600;
    let minutes = (whole % 3600) / 60;
    let remainder = whole % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{remainder:02}")
    } else {
        format!("{minutes}:{remainder:02}")
    }
}

pub fn present_video(metadata: VideoMetadata) -> VideoViewModel {
    let duration_label = duration_label(metadata.duration);
    VideoViewModel {
        metadata,
        duration_label,
    }
}

#[derive(Serialize)]
pub struct PublicError {
    pub code: String,
    pub message: String,
}

pub fn present_error(error: AppError) -> PublicError {
    PublicError {
        code: error.code.into(),
        message: error.message,
    }
}
