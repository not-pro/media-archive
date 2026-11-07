use std::path::Path;

use crate::domain::entities::DownloadOptions;
use crate::domain::error::AppResult;
use crate::domain::format_selector::build_format_selector;
use crate::infrastructure::engine::args::{SidecarPaths, base_args};
use crate::infrastructure::engine::output::{FILE_PREFIX, PROGRESS_PREFIX};

pub fn download_args(
    paths: &SidecarPaths,
    options: &DownloadOptions,
    cookies: Option<&Path>,
    url: &str,
) -> AppResult<Vec<String>> {
    let progress = format!(
        "download:{PROGRESS_PREFIX}%(progress.downloaded_bytes)s|%(progress.total_bytes)s|\
         %(progress.total_bytes_estimate)s|%(progress.speed)s|%(progress.eta)s"
    );
    let mut args = base_args(paths, cookies);
    args.extend([
        "--format".into(),
        build_format_selector(options.height)?,
        "--paths".into(),
        format!("home:{}", options.output_directory),
        "--output".into(),
        "%(title).180B [%(id)s].%(ext)s".into(),
        "--trim-filenames".into(),
        "240".into(),
        "--windows-filenames".into(),
        "--continue".into(),
        if options.overwrite {
            "--overwrites"
        } else {
            "--no-overwrites"
        }
        .into(),
        "--merge-output-format".into(),
        "mp4".into(),
        "--remux-video".into(),
        "mp4".into(),
        "--retries".into(),
        "3".into(),
        "--fragment-retries".into(),
        "3".into(),
        "--socket-timeout".into(),
        "30".into(),
        "--newline".into(),
        "--progress-delta".into(),
        "0.2".into(),
        "--progress-template".into(),
        progress,
        "--print".into(),
        format!("after_move:{FILE_PREFIX}%(filepath)j"),
        "--".into(),
        url.into(),
    ]);
    Ok(args)
}
