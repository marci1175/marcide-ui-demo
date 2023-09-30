#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use image::EncodableLayout;
mod setup;
use setup::write;

pub fn setup() {
    
    //drop image files on host desktop
    let mut names: Vec<&str> = Vec::new();
    let mut icons: Vec<&[u8]> = Vec::new();
    //file icon
    icons.push(include_bytes!("C:\\Users\\marci\\Desktop\\Marcide icons\\files.png").as_bytes());
    //name icon
    names.push("files.png");
                
    for i in 0..icons.len() {
        write(icons[i], names[i]);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "marcimenu",
        native_options,
        Box::new(|cc| Box::new(eframe_template::AppData::new(cc))),
    )
}


