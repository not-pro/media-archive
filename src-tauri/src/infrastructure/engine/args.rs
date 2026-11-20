use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct SidecarPaths {
    pub engine: PathBuf,
    pub runtime: PathBuf,
    pub merger: PathBuf,
    pub probe: PathBuf,
}

#[derive(Clone, Copy)]
pub enum Cookies<'a> {
    File(&'a Path),
    Browser(&'static str),
}

pub fn base_args(paths: &SidecarPaths, cookies: Option<Cookies<'_>>) -> Vec<String> {
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
    match cookies {
        Some(Cookies::File(file)) => {
            args.extend(["--cookies".into(), file.display().to_string()]);
        }
        Some(Cookies::Browser(browser)) => {
            args.extend(["--cookies-from-browser".into(), browser.into()]);
        }
        None => {}
    }
    args
}

pub fn inspect_args(paths: &SidecarPaths, cookies: Option<Cookies<'_>>, url: &str) -> Vec<String> {
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
