use tauri::AppHandle;
use tauri::Manager;
use tauri::RunEvent;
use tauri::WindowEvent;

use crate::core::configs::Configs;

pub fn run_handler(app: &AppHandle, event: RunEvent) {
    match event {
        // RunEvent::Exit => todo!(),
        // RunEvent::ExitRequested { api } => todo!(),
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::Resized(size),
            ..
        } => {
            let mut config = Configs::read_file();
            println!("{:?}", size);
            config.set_width(size.width).unwrap();
        }
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
