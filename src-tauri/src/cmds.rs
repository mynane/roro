use anyhow::Result;
use serde::{Deserialize, Serialize};

use tauri::{Size, State, Window};

use crate::{
    states::{CachesState, ConfigsState, TestState},
    utils::size::{change_width, init_position},
};
// use crate::wrap_err;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct SayOpt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_man: Option<bool>,
}

impl SayOpt {}

#[tauri::command]
pub fn say_hello(
    url: String,
    option: Option<SayOpt>,
    test_state: State<'_, TestState>,
    caches_state: State<'_, CachesState>,
) -> Result<String, String> {
    let mut caches = caches_state.0.lock().unwrap();
    #[allow(unused_mut)]
    let mut test = test_state.0.lock().unwrap();
    caches.age("122".to_string());
    println!("hello {}, {:?}, {:?}, {:?}", url, option, caches, test);
    // wrap_err!(Option())
    Ok(caches.clone().name)
}

#[tauri::command]
pub async fn show_dialog(_window: Window) -> Result<(), String> {
    // tauri::api::dialog::ask(Some(&window), "Tauri", "Is Tauri awesome?", |answer| {
    //     // do something with `answer`
    //     println!("nihoa");
    // });
    // tauri::api::dialog::confirm(Some(&window), "Tauri", "Is Tauri awesome?", |answer| {
    //     println!("{}", answer);
    // });

    // tauri::api::dialog::message(Some(&window), "Tauri", "Is Tauri awesome?");

    // FileDialogBuilder::new().pick_file(|file_path| {
    //     println!("{:?}", file_path);

    //     match file_path {
    //         Some(path) => println!("{:?}", path),
    //         None => println!("None"),
    //     }
    // });

    // FileDialogBuilder::new().pick_files(|file_paths| {
    //     println!("{:?}", file_paths);

    //     match file_paths {
    //         Some(paths) => {
    //             for path in paths {
    //                 println!("{:?}", path);
    //             }
    //         }
    //         None => {
    //             println!("none")
    //         }
    //     };
    // });

    // FileDialogBuilder::new()
    //     .set_title("你好")
    //     .pick_folder(|folder_path| {
    //         println!("{:?}", folder_path);
    //     });

    // FileDialogBuilder::new()
    //     .set_file_name("a.js")
    //     .save_file(|file_path| {
    //         println!("{:?}", file_path);
    //     });

    Ok(())
}

#[tauri::command]
pub fn unfold(window: Window, configs_state: State<'_, ConfigsState>) -> Result<(), String> {
    let mut configs = configs_state.0.lock().unwrap();
    configs.async_file().unwrap();
    configs.set_mode(false).unwrap();
    window.set_resizable(true).unwrap();

    let width = match configs.width {
        Some(width) => width,
        None => 400.0 as u32,
    };

    println!("{}", width);

    change_width(window, width).unwrap();

    Ok(())
}

#[tauri::command]
pub fn fold(window: Window, configs_state: State<'_, ConfigsState>) -> Result<(), String> {
    let mut configs = configs_state.0.lock().unwrap();
    configs.async_file().unwrap();
    configs.set_mode(true).unwrap();
    window.set_resizable(false).unwrap();

    println!("{:?}", 12);

    change_width(window, 30).unwrap();

    Ok(())
}

#[tauri::command]
pub fn isfold(_window: Window, configs_state: State<'_, ConfigsState>) -> Result<bool, String> {
    let mut configs = configs_state.0.lock().unwrap();
    configs.async_file().unwrap();

    let mode = configs.mode.unwrap_or_default();

    Ok(mode)
}
