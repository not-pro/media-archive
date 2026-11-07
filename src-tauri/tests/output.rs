use media_archive_lib::infrastructure::engine::metadata::parse_metadata;
use media_archive_lib::infrastructure::engine::output::{parse_output_file, parse_progress};

#[test]
fn parses_progress_and_output_path() {
    let progress = parse_progress("MEDIA_PROGRESS:50|200|NA|1024|15").unwrap();
    assert_eq!(progress.percent, Some(25.0));
    assert_eq!(progress.speed, Some(1024.0));
    assert_eq!(progress.eta, Some(15.0));
    assert!(parse_progress("[download] 50%").is_none());
    assert_eq!(
        parse_output_file("[download] MEDIA_FILE:\"/tmp/video.mp4\"").as_deref(),
        Some("/tmp/video.mp4")
    );
    assert_eq!(parse_output_file("MEDIA_FILE:\"relative.mp4\""), None);
}

#[test]
fn extracts_metadata_and_sorted_qualities() {
    let json = r#"{
        "id":"dQw4w9WgXcQ","title":"Example","uploader":"Owner","duration":12.0,
        "thumbnail":"https://example.com/image.jpg",
        "formats":[
            {"height":720,"vcodec":"avc1"},
            {"height":144,"vcodec":"avc1"},
            {"height":1080,"vcodec":"none"}
        ]
    }"#;
    let metadata = parse_metadata(json).unwrap();
    assert_eq!(metadata.title, "Example");
    let heights: Vec<u32> = metadata
        .qualities
        .iter()
        .map(|quality| quality.height)
        .collect();
    assert_eq!(heights, [720, 144]);
}

#[test]
fn rejects_live_metadata() {
    let json = r#"{"id":"dQw4w9WgXcQ","title":"Live","is_live":true}"#;
    assert_eq!(parse_metadata(json).unwrap_err().code, "VIDEO_UNSUPPORTED");
}
