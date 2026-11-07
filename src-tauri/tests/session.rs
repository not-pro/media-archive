use media_archive_lib::infrastructure::session::filter::filter_session_cookies;

fn cookie(domain: &str, name: &str) -> String {
    format!("{domain}\tTRUE\t/\tTRUE\t2147483647\t{name}\tsecret-value")
}

#[test]
fn keeps_only_valid_session_records() {
    let input = format!(
        "{}\n{}\n#HttpOnly_{}",
        cookie(".example.com", "OTHER"),
        cookie(".youtube.com", "SID"),
        cookie(".youtube.com", "HSID"),
    );
    let filtered = filter_session_cookies(&input).unwrap();
    assert!(filtered.contains("SID"));
    assert!(filtered.contains("#HttpOnly_.youtube.com"));
    assert!(!filtered.contains("example.com"));
}

#[test]
fn rejects_files_without_supported_records() {
    let error = filter_session_cookies(&cookie(".example.com", "OTHER")).unwrap_err();
    assert_eq!(error.code, "COOKIES_INVALID");
}
