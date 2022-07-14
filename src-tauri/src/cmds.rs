use anyhow::Result;
use serde::{Deserialize, Serialize};

use tauri::{Size, State, Window};

use crate::{
    core::{
        catalogue::{Catalogues, ClgItem},
        posts::{PostItem, Posts},
    },
    states::{CachesState, CataloguesState, ConfigsState, PostsState, TestState},
    utils::size::{change_width, init_position},
    wrap_err,
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

    change_width(window, width).unwrap();

    Ok(())
}

#[tauri::command]
pub fn fold(window: Window, configs_state: State<'_, ConfigsState>) -> Result<(), String> {
    let mut configs = configs_state.0.lock().unwrap();
    configs.async_file().unwrap();
    configs.set_mode(true).unwrap();
    window.set_resizable(false).unwrap();

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

/// get all catalogue from `catalogue.yaml`
#[tauri::command]
pub fn get_catalogues<'a>(
    catalogues_state: State<'_, CataloguesState>,
) -> Result<Catalogues, String> {
    let catalogues = catalogues_state.0.lock().unwrap();
    Ok(catalogues.clone())
}

/// synchronize data irregularly
#[tauri::command]
pub fn sync_catalogues(catalogues_state: State<'_, CataloguesState>) -> Result<(), String> {
    let mut catalogues = catalogues_state.0.lock().unwrap();
    wrap_err!(catalogues.sync_file())
}

/// synchronize data irregularly
#[tauri::command]
pub fn put_current(
    uid: String,
    catalogues_state: State<'_, CataloguesState>,
) -> Result<(), String> {
    let mut catalogues = catalogues_state.0.lock().unwrap();
    wrap_err!(catalogues.put_current(uid))
}

/// synchronize data irregularly1
// #[tauri::command]
// pub fn get_current(catalogues_state: State<'_, CataloguesState>) -> Result<()> {
//     let mut catalogues = catalogues_state.0.lock().unwrap();

//     // let result = catalogues.get_current();

//     Ok(())
// }

/// new a catalogue
/// append a temp catalogue item file to the `catalogues` dir
/// view the temp catalogue file by using vscode or other editor
#[tauri::command]
pub async fn create_catalogue(
    item: ClgItem,
    catalogues_state: State<'_, CataloguesState>,
) -> Result<(), String> {
    let mut catalogues = catalogues_state.0.lock().unwrap();
    wrap_err!(catalogues.append_item(item))
}

/// Update the catalogue
#[tauri::command]
pub async fn update_catalogue(
    uid: String,
    item: ClgItem,
    catalogues_state: State<'_, CataloguesState>,
) -> Result<(), String> {
    let mut catalogues = catalogues_state.0.lock().unwrap();
    wrap_err!(catalogues.patch_item(uid, item))?;

    Ok(())
}

/// delete catalogue item
#[tauri::command]
pub fn delete_catalogue(
    uid: String,
    catalogues_state: State<'_, CataloguesState>,
) -> Result<(), String> {
    let mut catalogues = catalogues_state.0.lock().unwrap();

    wrap_err!(catalogues.delete_item(uid))?;

    Ok(())
}

/// get all posts from `posts.yaml`
#[tauri::command]
pub fn get_posts(posts_state: State<'_, PostsState>) -> Result<Posts, String> {
    let posts = posts_state.0.lock().unwrap();
    Ok(posts.clone())
}

/// synchronize data irregularly
#[tauri::command]
pub fn sync_posts(posts_state: State<'_, PostsState>) -> Result<(), String> {
    let mut posts = posts_state.0.lock().unwrap();
    wrap_err!(posts.sync_file())
}

/// new a post
/// append a temp post item file to the `posts` dir
/// view the temp post file by using vscode or other editor
#[tauri::command]
pub async fn create_post(item: PostItem, posts_state: State<'_, PostsState>) -> Result<(), String> {
    let mut posts = posts_state.0.lock().unwrap();
    wrap_err!(posts.append_item(item))
}

/// Update the post
#[tauri::command]
pub async fn update_post(
    uid: String,
    item: PostItem,
    posts_state: State<'_, PostsState>,
) -> Result<(), String> {
    let mut posts = posts_state.0.lock().unwrap();
    wrap_err!(posts.patch_item(uid, item))?;

    Ok(())
}

/// delete post item
#[tauri::command]
pub fn delete_post(uid: String, posts_state: State<'_, PostsState>) -> Result<(), String> {
    let mut posts = posts_state.0.lock().unwrap();

    wrap_err!(posts.delete_item(uid))?;

    Ok(())
}

/// delete post by cuid
#[tauri::command]
pub fn delete_items_by_cuid(
    cuid: String,
    posts_state: State<'_, PostsState>,
) -> Result<(), String> {
    let mut posts = posts_state.0.lock().unwrap();

    wrap_err!(posts.delete_items_by_cuid(cuid))?;

    Ok(())
}

/// delete post by cuid
#[tauri::command]
pub fn get_items_by_cuid(
    cuid: String,
    posts_state: State<'_, PostsState>,
) -> Result<Vec<PostItem>, String> {
    let mut posts = posts_state.0.lock().unwrap();

    let posts = posts.get_items_by_cuid(cuid).unwrap();

    Ok(posts)
}
