use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::domain::error::{AppError, AppResult};
use crate::infrastructure::engine::args::SidecarPaths;
use crate::infrastructure::engine::target::sidecar_target;

pub fn sidecar_paths(app: &AppHandle) -> AppResult<SidecarPaths> {
    let target = sidecar_target();
    let directory = if cfg!(debug_assertions) {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../sidecars")
            .join(target)
    } else {
        app.path()
            .resource_dir()
            .map_err(|error| AppError::new("RESOURCE_ERROR", error.to_string()))?
            .join("sidecars")
    };
    let extension = if cfg!(windows) { ".exe" } else { "" };
    Ok(SidecarPaths {
        engine: directory.join(format!("yt-dlp{extension}")),
        runtime: directory.join(format!("deno{extension}")),
        merger: directory.join(format!("ffmpeg{extension}")),
        probe: directory.join(format!("ffprobe{extension}")),
    })
}

pub fn verify_sidecars(paths: &SidecarPaths) -> AppResult<()> {
    for executable in [&paths.engine, &paths.runtime, &paths.merger, &paths.probe] {
        if !executable.is_file() {
            return Err(AppError::new(
                "SIDECAR_MISSING",
                format!(
                    "Не найден {}. Выполните npm run sidecars.",
                    executable.display()
                ),
            ));
        }
    }
    Ok(())
}
