use anyhow::{Ok, Result};
use tauri::{Size, Window};

pub fn init_position(win: Window, wid: u32, mode: bool) -> Result<()> {
    let monitor = win.primary_monitor().unwrap();

    let width = match mode {
        true => 30,
        false => wid,
    };

    match monitor {
        Some(mo) => {
            let size = mo.size();
            // let mots = win.primary_monitor().unwrap();
            win.set_always_on_top(true).unwrap();
            // let scale_factor = mo.scale_factor();
            let position = tauri::PhysicalPosition::<i32> {
                x: (size.width - width as u32) as i32,
                y: 0,
            };

            win.set_size(Size::Physical(tauri::PhysicalSize {
                width: (width as f64) as u32,
                height: (size.height) as u32,
            }))
            .unwrap();

            win.set_position(tauri::Position::Physical(position))
                .unwrap();

            win.show().unwrap();
        }
        None => {}
    }
    println!("1");
    Ok(())
}

pub fn change_width(win: Window, width: u32) -> Result<()> {
    let monitor = win.primary_monitor().unwrap();
    match monitor {
        Some(mo) => {
            println!("{:?}", mo);
            let size = mo.size();
            let position = tauri::PhysicalPosition::<i32> {
                x: (size.width - width as u32) as i32,
                y: 0,
            };

            println!("{:?} {} {}", position, size.width, width);

            win.set_size(Size::Physical(tauri::PhysicalSize {
                width: (width as f64) as u32,
                height: (size.height) as u32,
            }))
            .unwrap();

            win.set_position(tauri::Position::Physical(position))
                .unwrap();
        }
        None => {}
    }
    println!("2");
    Ok(())
}
