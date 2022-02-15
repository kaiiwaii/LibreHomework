#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod db;
mod utils;
mod notify;

use tauri::{Manager, RunEvent};
use std::sync::atomic::{AtomicBool, Ordering};
use db::*;
use utils::*;
use notify::notify;


static IS_LOCKED: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn getScreenLock() -> bool {
  IS_LOCKED.load(Ordering::Relaxed)
}
#[tauri::command]
fn setScreenLock(val: bool) {
  IS_LOCKED.store(val, Ordering::Relaxed);
}


fn main() {

  let mut appdir = tauri::api::path::document_dir().unwrap();
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
      getScreenLock,
      setScreenLock,
      get_local_lang,
      addTask,
      notify
    ])
    .build(tauri::generate_context!())
    .expect("Error while running LibreHomework");

  app.run(|app_handle, event| match event {
    
    RunEvent::CloseRequested {label, api, ..} => {
      if IS_LOCKED.load(Ordering::Relaxed) {
        api.prevent_close()
      }
      else {
        app_handle.get_window(&label).unwrap().close().unwrap();
      }
    }

    _ => {}
  })
}
