pub mod chrome;

#[cfg(target_os = "macos")]
pub mod safari;

#[cfg(target_os = "macos")]
pub fn extract() -> crate::domain::error::AppResult<Option<String>> {
    // Try Chrome-family first, then Safari
    if let Ok(Some(cookies)) = chrome::extract() {
        return Ok(Some(cookies));
    }
    #[cfg(target_os = "macos")]
    if let Ok(Some(cookies)) = safari::extract() {
        return Ok(Some(cookies));
    }
    Ok(None)
}

#[cfg(not(target_os = "macos"))]
pub fn extract() -> crate::domain::error::AppResult<Option<String>> {
    chrome::extract()
}
