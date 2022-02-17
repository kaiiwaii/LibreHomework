use std::io::{Write, Read};
use std::fs::{OpenOptions, File, create_dir, remove_file};
use std::path::Path;
use tauri::api::path::document_dir;

pub fn stop_app(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(1)
}

#[tauri::command]
pub fn read_config_file() -> Option<String> {
    let mut file = File::open(document_dir()?.join("LibreHomework/config.json")).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    Some(contents)
}

#[tauri::command]
pub fn write_config_file(contents: &str) -> Option<()> {
    let path = document_dir()?.join("LibreHomework");

    if !Path::new(&path).exists() {
      create_dir(&path).ok()?;
    }
    if Path::new(&path.join("config.json")).exists() {
      remove_file(&path.join("config.json")).ok()?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.join("config.json"))
        .ok()?;
    file.write_all(contents.as_bytes()).ok()?;
    Some(())
}