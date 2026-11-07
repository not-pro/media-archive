use std::process::Child;

#[cfg(unix)]
pub fn kill_tree(child: &mut Child) {
    let pid = child.id() as i32;
    unsafe {
        libc::kill(-pid, libc::SIGTERM);
    }
    let _ = child.kill();
}

#[cfg(windows)]
pub fn kill_tree(child: &mut Child) {
    let result = std::process::Command::new("taskkill")
        .args(["/pid", &child.id().to_string(), "/T", "/F"])
        .spawn();
    if result.is_err() {
        let _ = child.kill();
    }
}
