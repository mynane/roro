use crate::core::configs::Configs;
use tauri::App;
use tauri::Manager;
use tauri::Monitor;
use tauri::Size;
use tauri::Window;
use tiny_http::Response;
use tiny_http::Server;

use crate::utils::init;

/// app setup
pub fn setup(app: &mut App) {
    let win = app.get_window("main").unwrap();
    let config = Configs::read_file();
    let width = match config.width {
        Some(width) => width,
        None => 400.0 as u32,
    };

    println!("{:?}", width);
    #[cfg(debug_assertions)]
    win.open_devtools();

    init::init_app(app.package_info());

    let monitor = win.current_monitor().unwrap();
    match monitor {
        Some(mo) => {
            let size = mo.size();
            let mots = win.primary_monitor().unwrap();
            println!("{:?}", mots);
            win.set_always_on_top(true).unwrap();
            // let scale_factor = mo.scale_factor();
            let position = tauri::PhysicalPosition::<i32> {
                x: (size.width - width as u32) as i32,
                y: 0,
            };

            win.set_size(Size::Physical(tauri::PhysicalSize {
                width: (width as f64) as u32,
                height: (size.height - 80.0 as u32) as u32,
            }))
            .unwrap();

            win.set_position(tauri::Position::Physical(position))
                .unwrap();

            win.show().unwrap();
        }
        None => {}
    }

    std::thread::spawn(|| {
        let server = Server::http("0.0.0.0:3003").unwrap();
        for request in server.incoming_requests() {
            println!(
                "received request! method: {:?}, url: {:?}, headers: {:?}",
                request.method(),
                request.url(),
                request.headers()
            );

            let response = Response::from_string("hello world");
            request.respond(response);
        }
    });
}
