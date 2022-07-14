use std::time::{SystemTime, UNIX_EPOCH};

use nanoid::nanoid;
use rand::random;
use tauri::{AppHandle, WindowBuilder};

pub fn get_now() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as _
}

const ALPHABET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// generate the uid
pub fn get_uid(prefix: &str) -> String {
    let id = nanoid!(11, &ALPHABET);
    format!("{prefix}{id}")
}

#[macro_export]
macro_rules! wrap_err {
    ($stat: expr) => {
        match $stat {
            Ok(a) => Ok(a),
            Err(err) => {
                log::error!("{}", err.to_string());
                Err(format!("{}", err.to_string()))
            }
        }
    };
}

pub fn create_child_window(app_handle: AppHandle) {
    let label = random::<i16>();
    WindowBuilder::new(
        &app_handle,
        label.to_string(),
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("Tauri - Rust")
    .build()
    .unwrap();
}
