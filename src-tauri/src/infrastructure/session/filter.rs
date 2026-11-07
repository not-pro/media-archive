use crate::domain::error::{AppError, AppResult};

pub fn filter_session_cookies(input: &str) -> AppResult<String> {
    let mut lines = Vec::new();
    for line in input.lines() {
        let http_only = line.starts_with("#HttpOnly_");
        let normalized = if http_only { &line[10..] } else { line };
        if line.starts_with('#') && !http_only {
            continue;
        }
        let fields: Vec<&str> = normalized.split('\t').collect();
        let domain = fields.first().unwrap_or(&"").to_lowercase();
        let allowed = domain == "youtube.com" || domain.ends_with(".youtube.com");
        if fields.len() == 7 && allowed {
            lines.push(line);
        }
    }
    if lines.is_empty() {
        return Err(AppError::new(
            "COOKIES_INVALID",
            "В файле нет подходящих cookies. Экспортируйте формат Netscape.",
        ));
    }
    Ok(format!(
        "# Netscape HTTP Cookie File\n{}\n",
        lines.join("\n")
    ))
}
