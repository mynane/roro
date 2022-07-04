use tauri::AppHandle;
use tauri::Manager;
use tauri::RunEvent;

pub fn run_handler(app: &AppHandle, event: RunEvent) {
    match event {
        // RunEvent::Exit => todo!(),
        // RunEvent::ExitRequested { api } => todo!(),
        // RunEvent::WindowEvent { label, event } => todo!(),
        // RunEvent::Ready => todo!(),
        // RunEvent::Resumed => todo!(),
        // RunEvent::MainEventsCleared => todo!(),
        // _ => todo!(),
        RunEvent::Ready => {
            let main_window = app.get_window("main").unwrap();

            main_window.listen("client-message", |event| {
                let data = event.payload();
                match data {
                    Some(data) => {
                        print!("{}", data);
                    }
                    None => {}
                }
                println!("{:?}", data);
            });
        }
        _ => {}
    }
}
