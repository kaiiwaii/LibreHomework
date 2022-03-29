use std::io::{Write, Read};
use std::fs::{OpenOptions, File, create_dir, remove_file};
use std::path::Path;
use tauri::api::path::config_dir;
use tauri::Manager;

pub fn stop_app(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(1)
}

#[tauri::command]
pub fn get_syslang() -> String {

  let locale = if let Ok(lang) = std::env::var("LANG") {
    lang[..2].to_string()
  } else {
    "en".to_string()
  };
  return locale

}

#[tauri::command]
pub fn read_config_file() -> Option<String> {
    let mut file = File::open(config_dir()?.join("LibreHomework/config.json")).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    Some(contents)
}

#[tauri::command]
pub fn write_config_file(contents: &str) -> Option<bool> {
    let path = config_dir()?.join("LibreHomework");

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
    Some(true)
}

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  
  window.get_window("main").unwrap().show().unwrap();
}