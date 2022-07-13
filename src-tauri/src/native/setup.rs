use crate::states;
use crate::utils::init;
use crate::utils::size::init_position;
use tauri::App;
use tauri::Manager;

/// app setup
pub fn setup(app: &mut App) {
    let win = app.get_window("main").unwrap();
    #[cfg(debug_assertions)]
    win.open_devtools();

    let configs_state = app.state::<states::ConfigsState>();
    let mut configs = configs_state.0.lock().unwrap();
    configs.async_file().unwrap();

    let width = match configs.width {
        Some(width) => width,
        None => 400.0 as u32,
    };

    // let mode = match configs.mode {
    //     Some(mode) => mode,
    //     None => "unfold".to_string(),
    // };

    let mode = configs.mode.unwrap_or_default();

    init_position(win, width, mode).unwrap();

    init::init_app(app.package_info());
}
