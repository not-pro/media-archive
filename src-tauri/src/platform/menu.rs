use tauri::menu::{Menu, MenuBuilder, SubmenuBuilder};
use tauri::{AppHandle, Wry};

pub fn fixed_window_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let application = SubmenuBuilder::new(app, "Media Archive")
        .about(None)
        .quit()
        .build()?;
    let edit = SubmenuBuilder::new(app, "Edit")
        .undo()
        .redo()
        .cut()
        .copy()
        .paste()
        .select_all()
        .build()?;
    MenuBuilder::new(app).items(&[&application, &edit]).build()
}
