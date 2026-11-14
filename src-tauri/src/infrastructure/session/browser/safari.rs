use std::path::PathBuf;

use crate::domain::error::{AppError, AppResult};

/// Safari binary cookies parser.
/// Format: pages of 4096 bytes, each page contains cookie entries with binary values.
const PAGE_SIZE: usize = 4096;
const SAFARI_EPOCH_OFFSET: u64 = 978_307_200; // seconds from Unix epoch (2001-01-01 to 1970-01-01)

struct SafariCookie {
    domain: String,
    name: String,
    value: String,
    path: String,
    secure: bool,
    #[allow(dead_code)]
    http_only: bool,
    expires: u64,
}

fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

fn read_cookie_value(
    data: &[u8],
    page_start: usize,
    value_offset: usize,
) -> AppResult<SafariCookie> {
    let vstart = page_start + value_offset;

    // Check minimum size for header
    if vstart + 10 > data.len() {
        return Err(AppError::new(
            "COOKIES_INVALID",
            "Не удалось прочитать cookies Safari.",
        ));
    }

    let _version = data[vstart];
    let flags = data[vstart + 1];
    let creation = u32::from_le_bytes([
        data[vstart + 2],
        data[vstart + 3],
        data[vstart + 4],
        data[vstart + 5],
    ]);
    let expiration = u32::from_le_bytes([
        data[vstart + 6],
        data[vstart + 7],
        data[vstart + 8],
        data[vstart + 9],
    ]);

    let domain_len = data[vstart + 10] as usize;
    let path_len = data[vstart + 11] as usize;

    let key_len = data[vstart + 12] as usize;
    let name_len = data[vstart + 13] as usize;
    let value_len = data[vstart + 14] as usize;

    // Key is after the header (null-terminated)
    let kstart = vstart + 15;
    if kstart + domain_len + 1 > data.len() {
        return Err(AppError::new(
            "COOKIES_INVALID",
            "Не удалось прочитать cookies Safari.",
        ));
    }
    let domain = extract_string(data, kstart, domain_len)?;

    // Path follows domain
    let pstart = kstart + domain_len + 1;
    if pstart + path_len + 1 > data.len() {
        return Err(AppError::new(
            "COOKIES_INVALID",
            "Не удалось прочитать cookies Safari.",
        ));
    }
    let path = extract_string(data, pstart, path_len)?;

    // Key (host_key) follows path - we skip it
    // Name follows key
    let kkey_start = pstart + path_len + 1;
    let nstart = kkey_start + key_len + 1;
    let name = extract_string(data, nstart, name_len)?;

    // Value follows name
    let vvalue_start = nstart + name_len + 1;
    let value_raw = if value_len > 0 && vvalue_start + value_len <= data.len() {
        extract_string(data, vvalue_start, value_len)?
    } else {
        String::new()
    };

    let expires = if expiration > 0 {
        (creation as u64) + SAFARI_EPOCH_OFFSET
    } else {
        0
    };

    Ok(SafariCookie {
        domain: domain
            .replace('\u{001f}', ".")
            .trim_start_matches('.')
            .to_string(),
        name,
        value: value_raw,
        path,
        secure: flags & 0x01 != 0,
        http_only: flags & 0x02 != 0,
        expires,
    })
}

fn extract_string(data: &[u8], start: usize, len: usize) -> AppResult<String> {
    if len == 0 || start + len > data.len() {
        return Ok(String::new());
    }
    let s = &data[start..start + len];
    // Remove null terminator if present
    let s = if s.ends_with(&[0]) {
        &s[..s.len() - 1]
    } else {
        s
    };
    String::from_utf8(s.to_vec()).map_err(|e| {
        AppError::new(
            "COOKIES_INVALID",
            format!("Invalid Safari cookie string: {e}"),
        )
    })
}

fn parse_page(data: &[u8], page_num: usize) -> AppResult<Vec<SafariCookie>> {
    let page_start = page_num * PAGE_SIZE;
    if page_start + 16 > data.len() {
        return Ok(Vec::new());
    }

    let _page_number = read_u32_le(data, page_start);
    let cookie_count = read_u32_le(data, page_start + 4);
    let first_entry_offset = read_u32_le(data, page_start + 8);

    if first_entry_offset == 0 {
        return Ok(Vec::new());
    }

    let mut cookies = Vec::new();
    let mut offset: u32 = first_entry_offset;

    for _ in 0..cookie_count {
        let offset_usize = offset as usize;
        if offset_usize < 16 || offset_usize + 20 > PAGE_SIZE {
            break;
        }

        let entry_abs = page_start + offset_usize;
        if entry_abs + 20 > data.len() {
            break;
        }

        let next_offset = read_u32_le(data, entry_abs);
        let _key_len = read_u32_le(data, entry_abs + 4);
        let value_offset = read_u32_le(data, entry_abs + 8);
        let _value_len = read_u32_le(data, entry_abs + 12);
        let _flags = read_u32_le(data, entry_abs + 16);

        let value_offset_usize = value_offset as usize;
        if value_offset == 0 || value_offset_usize >= PAGE_SIZE {
            break;
        }

        if let Ok(cookie) = read_cookie_value(data, page_start, value_offset_usize) {
            cookies.push(cookie);
        }

        offset = next_offset;
        if next_offset == 0 || next_offset <= offset {
            break;
        }
    }

    Ok(cookies)
}

fn to_netscape(cookies: &[SafariCookie]) -> String {
    let lines: Vec<String> = cookies
        .iter()
        .map(|c| {
            let effective_domain = if c.domain.starts_with('.') {
                c.domain[1..].to_string()
            } else {
                c.domain.clone()
            };

            let expires = if c.expires > 0 { c.expires } else { 0 };

            format!(
                "{flag}\t{domain}\t{path}\t{secure}\t{expires}\t{name}\t{value}",
                flag = if effective_domain.starts_with('.') {
                    "TRUE"
                } else {
                    "FALSE"
                },
                domain = effective_domain,
                path = c.path,
                secure = if c.secure { "TRUE" } else { "FALSE" },
                expires = expires,
                name = c.name,
                value = c.value,
            )
        })
        .collect();

    format!("# Netscape HTTP Cookie File\n{}\n", lines.join("\n"))
}

pub fn extract() -> AppResult<Option<String>> {
    let home = std::env::var("HOME").map_err(|_| {
        AppError::new(
            "BROWSER_NOT_FOUND",
            "Не удалось определить домашнюю директорию.",
        )
    })?;
    let cookies_path = PathBuf::from(format!("{home}/Library/Cookies/Cookies.binarycookies"));

    if !cookies_path.exists() {
        return Ok(None);
    }

    let data = std::fs::read(&cookies_path).map_err(|_| {
        AppError::new(
            "BROWSER_ACCESS_DENIED",
            "Не удалось прочитать cookies Safari. Закройте браузер и попробуйте снова.",
        )
    })?;

    let page_count = data.len() / PAGE_SIZE;
    let mut all_cookies = Vec::new();

    for i in 0..page_count {
        let page_cookies = parse_page(&data, i)?;
        all_cookies.extend(page_cookies);
    }

    if all_cookies.is_empty() {
        return Ok(None);
    }

    Ok(Some(to_netscape(&all_cookies)))
}
