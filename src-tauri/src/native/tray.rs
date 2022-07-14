use tauri::AppHandle;
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;

pub fn get_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("open_window", "打开"))
        .add_item(CustomMenuItem::new("exit_app", "Quit"));

    SystemTray::new().with_menu(tray_menu.clone())
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        // SystemTrayEvent::MenuItemClick { id } => todo!(),
        // SystemTrayEvent::LeftClick { position, size } => todo!(),
        // SystemTrayEvent::RightClick { position, size } => todo!(),
        // SystemTrayEvent::DoubleClick { position, size } => todo!(),
        // _ => todo!(),
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "open_window" => {
                // println!("shijinhua");
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        },
        #[cfg(target_os = "windows")]
        SystemTrayEvent::LeftClick { .. } => {
            // let window = app_handle.get_window("main").unwrap();
            // window.unminimize().unwrap();
            // window.show().unwrap();
            // window.set_focus().unwrap();
        }
        _ => {}
    }
}
