pub fn format_target(os: &str, arch: &str) -> String {
    let os = match os {
        "windows" => "win32",
        "macos" => "darwin",
        other => other,
    };
    let arch = match arch {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        other => other,
    };
    format!("{os}-{arch}")
}

pub fn sidecar_target() -> String {
    format_target(std::env::consts::OS, std::env::consts::ARCH)
}
