use std::path::Path;

use crate::domain::entities::{DownloadOptions, DownloadProgress, DownloadResult, VideoMetadata};
use crate::domain::error::AppResult;
use crate::domain::media_url::normalize_media_url;
use crate::infrastructure::engine::args::{SidecarPaths, inspect_args};
use crate::infrastructure::engine::download_args::download_args;
use crate::infrastructure::engine::error_mapper::map_engine_error;
use crate::infrastructure::engine::metadata::parse_metadata;
use crate::infrastructure::engine::output::{parse_output_file, parse_progress};
use crate::infrastructure::engine::process::MediaProcessRunner;

pub struct MediaGateway {
    paths: SidecarPaths,
    runner: MediaProcessRunner,
}

impl MediaGateway {
    pub fn new(paths: SidecarPaths) -> Self {
        Self {
            paths,
            runner: MediaProcessRunner::new(),
        }
    }

    pub fn inspect(&self, url: &str, cookies: Option<&Path>) -> AppResult<VideoMetadata> {
        let url = normalize_media_url(url)?;
        let args = inspect_args(&self.paths, cookies, &url);
        let result = self.runner.run(&self.paths.engine, &args, &|_| {})?;
        if result.exit_code != Some(0) {
            return Err(map_engine_error(&format!(
                "{}{}",
                result.stderr, result.stdout
            )));
        }
        parse_metadata(&result.stdout)
    }

    pub fn download(
        &self,
        options: &DownloadOptions,
        cookies: Option<&Path>,
        progress: &dyn Fn(DownloadProgress),
    ) -> AppResult<DownloadResult> {
        if !Path::new(&options.output_directory).is_absolute() {
            return Err(crate::domain::error::AppError::new(
                "DIRECTORY_INVALID",
                "Выберите папку сохранения.",
            ));
        }
        let url = normalize_media_url(&options.url)?;
        let args = download_args(&self.paths, options, cookies, &url)?;
        let result = self.runner.run(&self.paths.engine, &args, &|line| {
            if let Some(value) = parse_progress(line) {
                progress(value);
            }
        })?;
        if result.exit_code != Some(0) {
            return Err(map_engine_error(&format!(
                "{}{}",
                result.stderr, result.stdout
            )));
        }
        Ok(DownloadResult {
            file_path: parse_output_file(&result.stdout),
        })
    }

    pub fn cancel(&self) -> bool {
        self.runner.cancel()
    }
}
