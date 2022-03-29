#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![allow(non_snake_case)]

mod db;
mod models;
mod utils;
mod network;

use tauri::{RunEvent, WindowEvent, Manager};
use std::sync::atomic::{AtomicBool, Ordering};
use db::*;
use utils::*;
use network::*;

static IS_LOCKED: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn get_screen_lock() -> bool {
  IS_LOCKED.load(Ordering::Relaxed)
}
#[tauri::command]
fn set_screen_lock(val: bool) {
  IS_LOCKED.store(val, Ordering::Relaxed);
}


fn main() {

  let mut appdir = tauri::api::path::config_dir().unwrap();
  appdir.push("LibreHomework");

  if !appdir.exists() {
    
    match std::fs::create_dir(&appdir) {
      Err(e) => stop_app(&format!("Failed to create app directory: {}", e)),
      Ok(_) => {},
    }

  }
  appdir.push("LibreHomework.db");

  let app = tauri::Builder::default()
    .manage(DBManager::prepare_connection(appdir.as_path()))
    .invoke_handler(tauri::generate_handler![
      execute,
      get_screen_lock,
      set_screen_lock,
      add_task,
      remove_task,
      get_tasks,
      get_subjects,
      request,
      add_subject,
      remove_subject,
      write_config_file,
      read_config_file,
      close_splashscreen,
      get_syslang,
    ])
    .build(tauri::generate_context!())
    .expect("Error while running LibreHomework");

  app.run(|app_handle, event| match event {
    
    RunEvent::WindowEvent {label, event, ..} => {
      match event {
        WindowEvent::CloseRequested{api, ..} => {
          if IS_LOCKED.load(Ordering::Relaxed) {
            api.prevent_close();
          } else {
            app_handle.get_window(&label).unwrap().close().unwrap();
          }
        },
        _ => {}
      }
    },
      
    _ => {}
  })
}

