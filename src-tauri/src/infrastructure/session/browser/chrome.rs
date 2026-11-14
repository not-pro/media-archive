use std::path::PathBuf;

use base64::Engine;
use rusqlite::Connection;

use crate::domain::error::{AppError, AppResult};

/// Decode Chrome's b64 cookie value (standard base64 on macOS/Windows).
fn decode_chrome_value(encoded: &str) -> AppResult<String> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| AppError::new("COOKIES_INVALID", format!("Failed to decode cookie: {e}")))?;
    String::from_utf8(decoded)
        .map_err(|e| AppError::new("COOKIES_INVALID", format!("Invalid cookie encoding: {e}")))
}

struct ChromeCookie {
    domain: String,
    name: String,
    value: String,
    path: String,
    secure: bool,
    #[allow(dead_code)]
    http_only: bool,
}

#[cfg(not(target_os = "linux"))]
fn query_cookies(conn: &Connection) -> AppResult<Vec<ChromeCookie>> {
    let mut stmt = conn
        .prepare(
            "\
        SELECT host_key, name, value, path, expires_utc, secure, http_only \
        FROM cookies \
        WHERE host_key LIKE '%youtube%' OR host_key = '.google.com' OR host_key = 'google.com' \
        ORDER BY host_key, name
    ",
        )
        .map_err(|e| AppError::new("COOKIES_INVALID", format!("SQLite error: {e}")))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ChromeCookie {
                domain: row.get(0)?,
                name: row.get(1)?,
                value: row.get(2)?,
                path: row.get(3)?,
                secure: row.get(5)?,
                http_only: row.get(6)?,
            })
        })
        .map_err(|e| AppError::new("COOKIES_INVALID", format!("SQLite error: {e}")))?;

    Ok(rows.filter_map(|r| r.ok()).collect::<Vec<_>>())
}

#[cfg(target_os = "linux")]
fn query_cookies(conn: &Connection) -> AppResult<Vec<ChromeCookie>> {
    let mut stmt = conn
        .prepare(
            "\
        SELECT host_key, name, value, path, expiry_date, is_secure, is_http_only \
        FROM cookies \
        WHERE host_key LIKE '%youtube%' OR host_key = '.google.com' OR host_key = 'google.com' \
        ORDER BY host_key, name
    ",
        )
        .map_err(|e| AppError::new("COOKIES_INVALID", format!("SQLite error: {e}")))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ChromeCookie {
                domain: row.get(0)?,
                name: row.get(1)?,
                value: row.get(2)?,
                path: row.get(3)?,
                secure: row.get(5)?,
                http_only: row.get(6)?,
            })
        })
        .map_err(|e| AppError::new("COOKIES_INVALID", format!("SQLite error: {e}")))?;

    Ok(rows.filter_map(|r| r.ok()).collect::<Vec<_>>())
}

fn to_netscape(cookies: &[ChromeCookie]) -> String {
    let lines: Vec<String> = cookies
        .iter()
        .filter_map(|c| {
            // Determine effective domain and flag
            let effective_domain = if c.domain.starts_with('.') {
                c.domain[1..].to_string()
            } else {
                c.domain.clone()
            };

            // For cookies with b64 value (macOS/Windows), we need to decode
            let decoded_value = if cfg!(target_os = "linux") {
                Some(c.value.clone())
            } else {
                decode_chrome_value(&c.value).ok()
            };

            let value = decoded_value?;

            Some(format!(
                "{flag}\t{domain}\t{path}\t{secure}\t0\t{name}\t{value}",
                flag = if effective_domain.starts_with('.') {
                    "TRUE"
                } else {
                    "FALSE"
                },
                domain = effective_domain,
                path = c.path,
                secure = if c.secure { "TRUE" } else { "FALSE" },
                name = c.name,
                value = value,
            ))
        })
        .collect();

    format!("# Netscape HTTP Cookie File\n{}\n", lines.join("\n"))
}

#[cfg(target_os = "macos")]
pub fn extract() -> AppResult<Option<String>> {
    let home = std::env::var("HOME").map_err(|_| {
        AppError::new(
            "BROWSER_NOT_FOUND",
            "Не удалось определить домашнюю директорию.",
        )
    })?;

    let paths = [
        // Arc (Chrome-based)
        PathBuf::from(format!(
            "{home}/Library/Application Support/Arc/User Data/Default/Cookies"
        )),
        // Edge (Chrome-based)
        PathBuf::from(format!(
            "{home}/Library/Application Support/Microsoft Edge/Default/Cookies"
        )),
        // Brave
        PathBuf::from(format!(
            "{home}/Library/Application Support/BraveSoftware/Brave-Browser/Default/Cookies"
        )),
        // Chrome
        PathBuf::from(format!(
            "{home}/Library/Application Support/Google/Chrome/Default/Cookies"
        )),
    ];

    for path in &paths {
        if path.exists() {
            return try_extract(path);
        }
    }

    Ok(None)
}

#[cfg(target_os = "windows")]
pub fn extract() -> AppResult<Option<String>> {
    let local_app = std::env::var("LOCALAPPDATA").map_err(|_| {
        AppError::new(
            "BROWSER_NOT_FOUND",
            "Не удалось определить профиль пользователя.",
        )
    })?;

    let paths = [
        // Chrome
        PathBuf::from(format!(
            "{local_app}\\Google\\Chrome\\User Data\\Default\\Cookies"
        )),
        // Edge
        PathBuf::from(format!(
            "{local_app}\\Microsoft\\Edge\\User Data\\Default\\Cookies"
        )),
        // Brave
        PathBuf::from(format!(
            "{local_app}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Cookies"
        )),
    ];

    for path in &paths {
        if path.exists() {
            return try_extract(path);
        }
    }

    Ok(None)
}

#[cfg(target_os = "linux")]
pub fn extract() -> AppResult<Option<String>> {
    let home = std::env::var("HOME").map_err(|_| {
        AppError::new(
            "BROWSER_NOT_FOUND",
            "Не удалось определить домашнюю директорию.",
        )
    })?;
    let xdg_data = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(&home).join(".config"));

    let paths = [
        // Chrome
        PathBuf::from(format!("{home}/.config/google-chrome/Cookies")),
        // Edge
        PathBuf::from(format!("{home}/.config/microsoft-edge/Cookies")),
        // Brave
        PathBuf::from(format!(
            "{home}/.config/BraveSoftware/Brave-Browser/Cookies"
        )),
        // Chromium
        PathBuf::from(format!("{home}/.config/chromium/Cookies")),
        // Vivaldi
        PathBuf::from(format!("{home}/.config/vivaldi/Cookies")),
        // Opera
        xdg_data.join("Opera/Cookies"),
    ];

    for path in &paths {
        if path.exists() {
            return try_extract(path);
        }
    }

    Ok(None)
}

fn try_extract(source: &PathBuf) -> AppResult<Option<String>> {
    let conn = Connection::open(source).map_err(|_| {
        AppError::new(
            "BROWSER_ACCESS_DENIED",
            "Не удалось прочитать cookies браузера. Возможно, браузер должен быть закрыт.",
        )
    })?;

    let cookies = query_cookies(&conn)?;

    if cookies.is_empty() {
        return Ok(None);
    }

    Ok(Some(to_netscape(&cookies)))
}
