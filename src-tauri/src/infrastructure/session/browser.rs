use std::path::PathBuf;

pub fn available_browsers() -> impl Iterator<Item = &'static str> {
    #[cfg(target_os = "macos")]
    let profiles = [
        (
            "chrome",
            "HOME",
            "Library/Application Support/Google/Chrome",
        ),
        (
            "firefox",
            "HOME",
            "Library/Application Support/Firefox/Profiles",
        ),
        ("edge", "HOME", "Library/Application Support/Microsoft Edge"),
        (
            "brave",
            "HOME",
            "Library/Application Support/BraveSoftware/Brave-Browser",
        ),
        ("safari", "HOME", "Library/Containers/com.apple.Safari"),
    ];
    #[cfg(target_os = "windows")]
    let profiles = [
        ("firefox", "APPDATA", "Mozilla/Firefox/Profiles"),
        ("chrome", "LOCALAPPDATA", "Google/Chrome/User Data"),
        ("edge", "LOCALAPPDATA", "Microsoft/Edge/User Data"),
        (
            "brave",
            "LOCALAPPDATA",
            "BraveSoftware/Brave-Browser/User Data",
        ),
    ];
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let profiles = [
        ("firefox", "HOME", ".mozilla/firefox"),
        ("chrome", "HOME", ".config/google-chrome"),
        ("chromium", "HOME", ".config/chromium"),
    ];
    profiles
        .into_iter()
        .filter(|(_, variable, directory)| {
            std::env::var_os(variable)
                .is_some_and(|root| PathBuf::from(root).join(directory).is_dir())
        })
        .map(|(browser, _, _)| browser)
}
