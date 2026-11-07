use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct SidecarPaths {
    pub engine: PathBuf,
    pub runtime: PathBuf,
    pub merger: PathBuf,
    pub probe: PathBuf,
}

pub fn base_args(paths: &SidecarPaths, cookies: Option<&Path>) -> Vec<String> {
    let mut args = vec![
        "--ignore-config".into(),
        "--no-plugin-dirs".into(),
        "--no-remote-components".into(),
        "--no-playlist".into(),
        "--no-warnings".into(),
        "--js-runtimes".into(),
        format!("deno:{}", paths.runtime.display()),
        "--ffmpeg-location".into(),
        paths
            .merger
            .parent()
            .unwrap_or(Path::new(""))
            .display()
            .to_string(),
        "--extractor-args".into(),
        "youtube:fetch_pot=never".into(),
    ];
    if let Some(file) = cookies {
        args.extend(["--cookies".into(), file.display().to_string()]);
    }
    args
}

pub fn inspect_args(paths: &SidecarPaths, cookies: Option<&Path>, url: &str) -> Vec<String> {
    let mut args = base_args(paths, cookies);
    args.extend([
        "--dump-single-json".into(),
        "--skip-download".into(),
        "--no-progress".into(),
        "--".into(),
        url.into(),
    ]);
    args
}
