#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

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

fn make_executable(path: &Path) {
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

#[test]
fn parses_process_events() {
    let root = tempfile::tempdir().unwrap();
    let executable = root.path().join("fake-engine");
    fs::write(
        &executable,
        "#!/bin/sh\necho 'MEDIA_PROGRESS:50|100|100|1024|5'\necho 'MEDIA_FILE:\"/tmp/final.mp4\"'\n",
    )
    .unwrap();
    make_executable(&executable);
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
    make_executable(&executable);
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

#[test]
fn rejects_overlapping_runs() {
    let root = tempfile::tempdir().unwrap();
    let executable = root.path().join("fake-engine");
    fs::write(
        &executable,
        "#!/bin/sh\necho 'MEDIA_FILE:\"/tmp/final.mp4\"'\n",
    )
    .unwrap();
    make_executable(&executable);
    let gateway = std::sync::Arc::new(gateway(&executable));

    let worker_one = {
        let gateway = gateway.clone();
        let root = root.path().to_path_buf();
        std::thread::spawn(move || gateway.download(&options(&root), None, &|_| {}))
    };
    let worker_two = {
        let gateway = gateway.clone();
        let root = root.path().to_path_buf();
        std::thread::spawn(move || gateway.download(&options(&root), None, &|_| {}))
    };

    let results = [worker_one.join().unwrap(), worker_two.join().unwrap()];
    let successes = results.iter().filter(|r| r.is_ok()).count();
    let busy = results
        .iter()
        .filter(|r| matches!(r.as_ref(), Err(e) if e.code == "DOWNLOAD_BUSY"))
        .count();
    assert_eq!(successes, 1);
    assert_eq!(busy, 1);
}
