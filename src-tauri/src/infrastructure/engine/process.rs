use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::domain::error::{AppError, AppResult};
use crate::infrastructure::engine::process_kill::kill_tree;

pub struct ProcessResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

struct ActiveGuard<'a> {
    active: &'a AtomicBool,
}

impl Drop for ActiveGuard<'_> {
    fn drop(&mut self) {
        self.active.store(false, Ordering::SeqCst);
    }
}

#[derive(Default)]
pub struct MediaProcessRunner {
    child: Arc<Mutex<Option<Child>>>,
    cancelled: Arc<AtomicBool>,
    active: Arc<AtomicBool>,
}

impl MediaProcessRunner {
    pub fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
            cancelled: Arc::new(AtomicBool::new(false)),
            active: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) -> bool {
        self.cancelled.store(true, Ordering::SeqCst);
        let mut guard = self.child.lock().unwrap();
        if let Some(child) = guard.as_mut() {
            kill_tree(child);
            true
        } else {
            false
        }
    }

    pub fn run(
        &self,
        executable: &Path,
        args: &[String],
        on_line: &dyn Fn(&str),
    ) -> AppResult<ProcessResult> {
        if self.active.swap(true, Ordering::SeqCst) {
            return Err(AppError::new(
                "DOWNLOAD_BUSY",
                "Сначала отмените текущее скачивание.",
            ));
        }
        let _active_guard = ActiveGuard {
            active: &self.active,
        };
        self.cancelled.store(false, Ordering::SeqCst);
        let mut command = Command::new(executable);
        command
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command.env("PYTHONIOENCODING", "utf-8");
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        let mut child = command.spawn().map_err(|error| {
            AppError::new(
                "SIDECAR_START_FAILED",
                format!("Не удалось запустить загрузчик: {error}"),
            )
        })?;
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        *self.child.lock().unwrap() = Some(child);
        let stderr_task = thread::spawn(move || {
            let mut text = String::new();
            let _ = BufReader::new(stderr).read_to_string(&mut text);
            text
        });

        let mut stdout_text = String::new();
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if stdout_text.len() < 64 * 1024 * 1024 {
                stdout_text.push_str(&line);
                stdout_text.push('\n');
            }
            on_line(&line);
        }
        let mut child = self.child.lock().unwrap().take().unwrap();
        let status = child.wait();
        let stderr_text = stderr_task.join().unwrap_or_default();
        if self.cancelled.load(Ordering::SeqCst) {
            return Err(AppError::new(
                "CANCELLED",
                "Скачивание отменено. Повторный запуск продолжит файл.",
            ));
        }
        let status = status.map_err(|error| AppError::new("PROCESS_FAILED", error.to_string()))?;
        Ok(ProcessResult {
            stdout: stdout_text,
            stderr: stderr_text,
            exit_code: status.code(),
        })
    }
}
