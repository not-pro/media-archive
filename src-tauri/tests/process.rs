#![cfg(unix)]

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use media_archive_lib::application::ports::VideoGateway;
use media_archive_lib::domain::entities::DownloadOptions;
use media_archive_lib::infrastructure::engine::args::SidecarPaths;
use media_archive_lib::infrastructure::engine::gateway::MediaGateway;

fn gateway(executable: &Path) -> MediaGateway {
    MediaGateway::new(SidecarPaths {
        engine: executable.into(),
        runtime: PathBuf::from("runtime"),
        merger: PathBuf::from("/tmp/merger"),
        probe: PathBuf::from("/tmp/probe"),
    })
}

fn options(root: &Path) -> DownloadOptions {
    DownloadOptions {
        url: "https://youtu.be/dQw4w9WgXcQ".into(),
        height: None,
        output_directory: root.display().to_string(),
        overwrite: false,
    }
}

#[test]
fn parses_process_events() {
    let root = tempfile::tempdir().unwrap();
    let executable = root.path().join("fake-engine");
    let script = "#!/bin/sh\necho 'MEDIA_PROGRESS:50|100|100|1024|5'\necho 'MEDIA_FILE:\"/tmp/final.mp4\"'\n";
    fs::write(&executable, script).unwrap();
    let mut permissions = fs::metadata(&executable).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
    fs::set_permissions(&executable, permissions).unwrap();
    let result = gateway(&executable)
        .download(&options(root.path()), None, &|progress| {
            assert_eq!(progress.percent, Some(50.0));
        })
        .unwrap();
    assert_eq!(result.file_path.as_deref(), Some("/tmp/final.mp4"));
}

#[test]
fn cancels_process_group() {
    let root = tempfile::tempdir().unwrap();
    let executable = root.path().join("fake-engine");
    fs::write(&executable, "#!/bin/sh\nwhile true; do sleep 1; done\n").unwrap();
    let mut permissions = fs::metadata(&executable).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
    fs::set_permissions(&executable, permissions).unwrap();
    let gateway = std::sync::Arc::new(gateway(&executable));
    let worker = {
        let gateway = gateway.clone();
        let root = root.path().to_path_buf();
        std::thread::spawn(move || gateway.download(&options(&root), None, &|_| {}))
    };
    std::thread::sleep(Duration::from_millis(150));
    assert!(gateway.cancel());
    let error = worker.join().unwrap().unwrap_err();
    assert_eq!(error.code, "CANCELLED");
}
