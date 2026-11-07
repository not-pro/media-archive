use std::path::Path;

use crate::domain::entities::DownloadProgress;

pub const PROGRESS_PREFIX: &str = "MEDIA_PROGRESS:";
pub const FILE_PREFIX: &str = "MEDIA_FILE:";

fn number(value: &str) -> Option<f64> {
    if value.is_empty() || matches!(value, "NA" | "None") {
        return None;
    }
    value.parse().ok().filter(|n: &f64| n.is_finite())
}

pub fn parse_progress(line: &str) -> Option<DownloadProgress> {
    let values: Vec<&str> = line.strip_prefix(PROGRESS_PREFIX)?.split('|').collect();
    let downloaded = number(values.first()?);
    let total = number(values.get(1)?).or_else(|| number(values.get(2)?));
    let percent = match (downloaded, total) {
        (Some(value), Some(total)) if total > 0.0 => {
            Some((value / total * 100.0).clamp(0.0, 100.0))
        }
        _ => None,
    };
    Some(DownloadProgress {
        percent,
        speed: values.get(3).and_then(|v| number(v)),
        eta: values.get(4).and_then(|v| number(v)),
    })
}

pub fn parse_output_file(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        let Some(index) = line.find(FILE_PREFIX) else {
            continue;
        };
        let value = &line[index + FILE_PREFIX.len()..];
        let Ok(path) = serde_json::from_str::<String>(value) else {
            return None;
        };
        if Path::new(&path).is_absolute() {
            return Some(path);
        }
    }
    None
}
