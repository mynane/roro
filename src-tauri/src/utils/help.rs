use rand::random;
use tauri::{AppHandle, WindowBuilder};

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
    println!("{}", label);
    WindowBuilder::new(
        &app_handle,
        label.to_string(),
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("Tauri - Rust")
    .build()
    .unwrap();
}
