#[tauri::command]
pub fn get_local_lang() -> Option<String> {
    if let Ok(e) = std::env::var("LANG") {
        Some(e)
    } else {
        None
    }
}

pub fn stop_app(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(1)
  }