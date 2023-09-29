use std::fs;

use egui::TextBuffer;
pub fn main(bytes : &[u8]) {
    let path = "C:\\Users\\".to_owned() + std::env::var_os("USER").unwrap().to_string_lossy().as_str() + "\\AppData\\Roaming\\marcimenu";
    match fs::create_dir(path){
        Ok(_) => {},
        Err(_) => {}
    };
}