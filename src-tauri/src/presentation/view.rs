use crate::domain::error::AppError;

#[derive(serde::Serialize)]
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
