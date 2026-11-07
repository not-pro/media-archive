use media_archive_lib::domain::format_selector::build_format_selector;
use media_archive_lib::domain::media_url::normalize_media_url;

#[test]
fn normalizes_supported_urls() {
    let expected = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    assert_eq!(
        normalize_media_url("https://youtu.be/dQw4w9WgXcQ?t=42").unwrap(),
        expected
    );
    assert_eq!(
        normalize_media_url("https://www.youtube.com/shorts/dQw4w9WgXcQ").unwrap(),
        expected
    );
    assert_eq!(
        normalize_media_url("https://music.youtube.com/watch?v=dQw4w9WgXcQ&list=x").unwrap(),
        expected
    );
}

#[test]
fn rejects_invalid_urls() {
    for url in [
        "https://example.com/watch?v=dQw4w9WgXcQ",
        "http://youtube.com/watch?v=dQw4w9WgXcQ",
        "https://user:pass@youtube.com/watch?v=dQw4w9WgXcQ",
        "https://youtube.com/playlist?list=abc",
        "https://youtube.com/watch?v=short",
    ] {
        assert_eq!(normalize_media_url(url).unwrap_err().code, "URL_INVALID");
    }
}

#[test]
fn builds_only_bounded_format_selectors() {
    assert_eq!(build_format_selector(None).unwrap(), "bv*+ba/b");
    assert_eq!(
        build_format_selector(Some(720)).unwrap(),
        "bv*[height<=720]+ba/b[height<=720]"
    );
    assert_eq!(
        build_format_selector(Some(0)).unwrap_err().code,
        "FORMAT_INVALID"
    );
    assert_eq!(
        build_format_selector(Some(9000)).unwrap_err().code,
        "FORMAT_INVALID"
    );
}
