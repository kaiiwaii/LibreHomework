use crate::errors::ConfigError;
use std::fs::{create_dir, remove_file, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use tauri::api::path::config_dir;

#[tauri::command]
pub fn get_syslang() -> String {
    let locale = if let Ok(lang) = std::env::var("LANG") {
        lang[..2].to_string()
    } else {
        "en".to_string()
    };
    return locale;
}

#[tauri::command]
pub fn read_config_file() -> Result<String, ConfigError> {
    let mut file = File::open(
        config_dir()
            .ok_or(ConfigError::ReadError)?
            .join("LibreHomework/config.json"),
    )
    .map_err(|_| ConfigError::ReadError)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| ConfigError::ReadError)?;
    Ok(contents)
}

#[tauri::command]
pub fn write_config_file(contents: &str) -> Result<(), ConfigError> {
    let path = config_dir()
        .ok_or(ConfigError::WriteError)?
        .join("LibreHomework");

    if !Path::new(&path).exists() {
        create_dir(&path).map_err(|_| ConfigError::WriteError)?;
    }
    if Path::new(&path.join("config.json")).exists() {
        remove_file(&path.join("config.json")).map_err(|_| ConfigError::WriteError)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.join("config.json"))
        .map_err(|_| ConfigError::WriteError)?;
    file.write_all(contents.as_bytes())
        .map_err(|_| ConfigError::WriteError)?;
    Ok(())
}

// #[tauri::command]
// pub async fn close_splashscreen(window: tauri::Window) {
//   if let Some(splashscreen) = window.get_window("splashscreen") {
//     splashscreen.close().unwrap();
//   }

//   window.get_window("main").unwrap().show().unwrap();
// }
