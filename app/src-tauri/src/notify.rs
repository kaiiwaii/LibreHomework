use notify_rust::{Notification};

#[tauri::command]
pub fn notify(title: &str, message: &str) {
    
    Notification::new()
    .summary(title)
    .body(message)
    .appname("LibreHomework")
    .timeout(0) 
    .show().expect("Failed to send notification");
}