use serde_json;
use serde::Deserialize;
use dirs_next::document_dir;
use std::collections::HashMap;



#[derive(Debug, Clone, Deserialize)]
#[serde(default = "default_daemon_config")]
pub struct DaemonConfig {
    pub start_notification: Option<bool>,
    pub notify_on_error: Option<bool>,
    pub notify_on_warning: Option<bool>,
    pub remind_every: Option<u32>, //minutes
    pub log_path: Option<String>,
}

impl DaemonConfig {
    pub fn read_config() -> Option<DaemonConfig> {
        let path = document_dir()?.join("LibreHomework/config.json");
        let config_file = std::fs::read_to_string(path).ok()?;

        serde_json::from_str::<HashMap<&str, DaemonConfig>>(&config_file).ok()?.get("daemon").cloned()
    }
}

fn default_daemon_config() -> DaemonConfig {
    let path: String = document_dir().unwrap().join("LibreHomework/librehomeworkd.log").to_str().unwrap().to_string();
    DaemonConfig {
        start_notification: Some(true),
        notify_on_error: Some(true),
        notify_on_warning: Some(true),
        remind_every: Some(30),
        log_path: Some(path),
    }
}