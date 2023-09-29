use std::fs;
use std::fs::File;
use std::io::Write;
use egui::TextBuffer;

pub fn write(bytes : &[u8], file_name : &str) {
    let path: &str = &("C:\\Users\\".to_owned() + &whoami::username() + "\\AppData\\Roaming\\marcimenu");
    dbg!(path);
    //makefolder
    match fs::create_dir(path){
        Ok(_) => {},
        Err(_) => {}
    };

    let mut file = match File::create(format!("{}\\{}", path, file_name)) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    // Write the bytes to the file
    match file.write_all(bytes) {
        Ok(_) => println!("File successfully written."),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }

}