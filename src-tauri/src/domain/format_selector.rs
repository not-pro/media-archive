use crate::domain::error::{AppError, AppResult};

pub fn build_format_selector(height: Option<u32>) -> AppResult<String> {
    match height {
        None => Ok("bv*+ba/b".into()),
        Some(value) if (144..=4320).contains(&value) => {
            Ok(format!("bv*[height<={value}]+ba/b[height<={value}]"))
        }
        Some(_) => Err(AppError::new(
            "FORMAT_INVALID",
            "Выбрано некорректное качество видео.",
        )),
    }
}
