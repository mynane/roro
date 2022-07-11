use anyhow::Result;
use serde::{Deserialize, Serialize};

use tauri::{State, Window};

use crate::states::{CachesState, TestState};
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
