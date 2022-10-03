#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(non_snake_case)]

mod db;
mod errors;
mod exams;
mod models;
mod network;
mod tasks;
mod utils;

use db::*;
use errors::BootError;
use exams::*;
use network::*;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use tasks::*;
use tauri::{Manager, RunEvent, WindowEvent};
use utils::*;

static IS_LOCKED: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn get_screen_lock() -> bool {
    IS_LOCKED.load(Ordering::Relaxed)
}
#[tauri::command]
fn set_screen_lock(val: bool) {
    IS_LOCKED.store(val, Ordering::Relaxed);
}

fn main() -> Result<(), Box<dyn Error>> {
    let appdir = tauri::api::path::config_dir();
    if appdir.is_none() {
        return Err(Box::new(BootError::ConfigDirError));
    }
    let mut appdir = appdir.unwrap();
    appdir.push("LibreHomework");

    if !appdir.exists() {
        match std::fs::create_dir(&appdir) {
            Err(_) => return Err(Box::new(BootError::CreateAppDirError)),
            Ok(_) => write_config_file(
                r##"{ "misc": { "lang": "en" }, "colors": { "primary": "#3942ed", "secondary": "5056c7" } }"##,
            )?,
        };
    }
    appdir.push("LibreHomework.db");
    let app = tauri::Builder::default()
        .manage(DBManager::prepare_connection(appdir.as_path())?)
        .invoke_handler(tauri::generate_handler![
            get_screen_lock,
            set_screen_lock,
            add_task,
            remove_task,
            get_tasks,
            get_subjects,
            get_task_documents,
            request,
            add_subject,
            remove_subject,
            write_config_file,
            read_config_file,
            get_syslang,
            get_exams,
            add_exam,
            remove_exam
        ])
        .build(tauri::generate_context!())
        .expect("Error while running LibreHomework");

    app.run(|app_handle, event| match event {
        RunEvent::WindowEvent { label, event, .. } => match event {
            WindowEvent::CloseRequested { api, .. } => {
                if IS_LOCKED.load(Ordering::Relaxed) {
                    api.prevent_close();
                } else {
                    app_handle.get_window(&label).unwrap().close().unwrap();
                }
            }
            _ => {}
        },

        _ => {}
    });
    Ok(())
}
