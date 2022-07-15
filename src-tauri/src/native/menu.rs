use std::path::PathBuf;

use tauri::{
    api::dialog::FileDialogBuilder, CustomMenuItem, Manager, Menu, MenuItem, Submenu,
    WindowMenuEvent,
};

/// get menu
pub fn get_menu() -> Menu {
    // #[allow(unused_mut)]
    // let mut disable_item =
    //     CustomMenuItem::new("disable-menu", "Disable menu").accelerator("CmdOrControl+D");
    // #[allow(unused_mut)]
    // let mut test_item = CustomMenuItem::new("test", "Test").accelerator("CmdOrControl+T");
    // #[cfg(target_os = "macos")]
    // {
    //     disable_item = disable_item.native_image(tauri::NativeImage::MenuOnState);
    //     test_item = test_item.native_image(tauri::NativeImage::Add);
    // }
    let about_menu = Submenu::new(
        "App",
        Menu::new()
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    );

    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    let view_menu = Submenu::new(
        "View",
        Menu::new().add_native_item(MenuItem::EnterFullScreen),
    );

    let window_menu = Submenu::new(
        "Window",
        Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::Zoom),
    );

    let help_menu = Submenu::new(
        "Help",
        Menu::new().add_item(CustomMenuItem::new("Learn More", "Learn More")),
    );

    Menu::new()
        .add_submenu(about_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(window_menu)
        .add_submenu(help_menu)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Payload {
    pub message: PathBuf,
}

/// on menu event
pub fn on_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "open-folder" => {
            FileDialogBuilder::new().pick_folder(move |folder_path| match folder_path {
                Some(path) => {
                    let main_window = event.window().get_window("main").unwrap();
                    main_window
                        .emit("on-target-change", Payload { message: path })
                        .unwrap();
                }
                None => {}
            });
        }
        id => {
            // do something with other events
            // println!("got menu event: {}", id);
        }
    }
}
