#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmds;
mod core;
mod native;
mod states;
mod utils;

use native::{menu, run, setup, tray};

fn main() {
    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        .manage(states::CachesState::default())
        .manage(states::TestState::default())
        .manage(states::ConfigsState::default())
        .setup(|app| Ok(setup::setup(app)))
        .on_page_load(|window, _| {
            let _window = window.clone();
            window.listen("js-event", move |_event| {
                // println!("got js-event with message '{:?}'", event.payload());
                // let reply = Reply {
                //     data: "something else".to_string(),
                // };

                // window_
                //     .emit("rust-event", Some(reply))
                //     .expect("failed to emit");
            });
        })
        .menu(menu::get_menu())
        .on_menu_event(menu::on_menu_event)
        .system_tray(tray::get_tray())
        .on_system_tray_event(tray::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            cmds::say_hello,
            cmds::show_dialog,
            cmds::fold,
            cmds::unfold,
            cmds::isfold,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(run::run_handler);
}
