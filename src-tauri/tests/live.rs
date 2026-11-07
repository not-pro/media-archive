use std::fs;
use std::path::PathBuf;
use std::process::Command;

use media_archive_lib::application::ports::VideoGateway;
use media_archive_lib::domain::entities::DownloadOptions;
use media_archive_lib::infrastructure::engine::args::SidecarPaths;
use media_archive_lib::infrastructure::engine::gateway::MediaGateway;
use media_archive_lib::infrastructure::engine::target::sidecar_target;

fn paths() -> SidecarPaths {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../sidecars");
    let target = sidecar_target();
    let directory = root.join(target);
    let extension = if cfg!(windows) { ".exe" } else { "" };
    SidecarPaths {
        engine: directory.join(format!("yt-dlp{extension}")),
        runtime: directory.join(format!("deno{extension}")),
        merger: directory.join(format!("ffmpeg{extension}")),
        probe: directory.join(format!("ffprobe{extension}")),
    }
}

#[test]
#[ignore]
fn downloads_authorized_video() {
    let url = std::env::var("MEDIA_LIVE_URL").expect("MEDIA_LIVE_URL is required");
    let paths = paths();
    let gateway = MediaGateway::new(paths.clone());
    let metadata = gateway.inspect(&url, None).unwrap();
    println!("{} — {}", metadata.uploader, metadata.title);
    let output = std::env::var("MEDIA_LIVE_OUTPUT").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("media-archive-live")
            .display()
            .to_string()
    });
    fs::create_dir_all(&output).unwrap();
    let result = gateway
        .download(
            &DownloadOptions {
                url,
                height: Some(144),
                output_directory: output,
                overwrite: false,
            },
            None,
            &|progress| {
                if let Some(percent) = progress.percent {
                    println!("{percent:.1}%");
                }
            },
        )
        .unwrap();
    let file = result.file_path.expect("No output path returned");
    assert!(fs::metadata(&file).unwrap().len() > 0);
    let probe = Command::new(paths.probe)
        .args([
            "-v",
            "error",
            "-show_entries",
            "stream=codec_type",
            "-of",
            "json",
            &file,
        ])
        .output()
        .unwrap();
    assert!(probe.status.success());
    let streams = String::from_utf8_lossy(&probe.stdout);
    assert!(streams.contains("video") && streams.contains("audio"));
    println!("Saved: {file}");
}
