use media_archive_lib::infrastructure::engine::target::format_target;

#[test]
fn maps_rust_target_names_to_manifest_names() {
    assert_eq!(format_target("macos", "aarch64"), "darwin-arm64");
    assert_eq!(format_target("macos", "x86_64"), "darwin-x64");
    assert_eq!(format_target("windows", "x86_64"), "win32-x64");
}
