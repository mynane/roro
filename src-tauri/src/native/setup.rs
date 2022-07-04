use tauri::App;
use tauri::Manager;
use tiny_http::Response;
use tiny_http::Server;

/// app setup
pub fn setup(app: &mut App) {
    let win = app.get_window("main").unwrap();
    #[cfg(debug_assertions)]
    win.open_devtools();

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
