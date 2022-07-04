use std::path::PathBuf;

use tauri::{
    api::dialog::FileDialogBuilder, CustomMenuItem, Manager, Menu, MenuItem, Submenu,
    WindowMenuEvent,
};

/// get menu
pub fn get_menu() -> Menu {
    #[allow(unused_mut)]
    let mut disable_item =
        CustomMenuItem::new("disable-menu", "Disable menu").accelerator("CmdOrControl+D");
    #[allow(unused_mut)]
    let mut test_item = CustomMenuItem::new("test", "Test").accelerator("CmdOrControl+T");
    #[cfg(target_os = "macos")]
    {
        disable_item = disable_item.native_image(tauri::NativeImage::MenuOnState);
        test_item = test_item.native_image(tauri::NativeImage::Add);
    }

    // create a submenu
    let my_sub_menu = Menu::with_items([disable_item.into()]);

    let my_app_menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(Submenu::new("Sub menu", my_sub_menu));

    let test_menu = Menu::new()
        .add_item(CustomMenuItem::new("open-folder", "打开文件夹"))
        .add_native_item(MenuItem::Separator)
        .add_item(test_item);

    // add all our childs to the menu (order is how they'll appear)
    Menu::new()
        .add_submenu(Submenu::new("My app", my_app_menu))
        .add_submenu(Submenu::new("文件", test_menu))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Payload {
    pub message: PathBuf,
}

/// on menu event
pub fn on_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "open-folder" => {
            FileDialogBuilder::new().pick_folder(move |folder_path| {
                println!("{:?}", folder_path);
                match folder_path {
                    Some(path) => {
                        let main_window = event.window().get_window("main").unwrap();
                        main_window
                            .emit("on-target-change", Payload { message: path })
                            .unwrap();
                    }
                    None => {}
                }
            });
        }
        id => {
            // do something with other events
            println!("got menu event: {}", id);
        }
    }
}
