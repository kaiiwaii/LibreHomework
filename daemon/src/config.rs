use serde_json;
use serde::{Deserialize, Serialize};
use dirs_next::config_dir;
use std::collections::HashMap;



#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default = "default_daemon_config")]
pub struct DaemonConfig {
    pub start_notification: Option<bool>,
    pub notify_on_error: Option<bool>,
    pub notify_on_warning: Option<bool>,
    pub remind_every: Option<u32>, //minutes
    pub remind_limit: Option<u32>, 
    pub log_path: Option<String>,
}

impl DaemonConfig {
    pub fn read_config() -> Option<DaemonConfig> {
        let path = config_dir()?.join("LibreHomework/daemonconfig.json");
        if !path.exists() {
            println!("Daemon config file not found, creating default config");
            DaemonConfig::write_config(default_daemon_config());
        }
        let config_file = std::fs::read_to_string(path).ok()?;
        serde_json::from_str::<HashMap<&str, DaemonConfig>>(&config_file).ok()?.get("daemon").cloned()
    }

    pub fn write_config(config: DaemonConfig) -> Option<bool> {
        let path = config_dir()?.join("LibreHomework/daemonconfig.json");

        let mut config_map= HashMap::new();
        config_map.insert("daemon", config);

        let config_json = serde_json::to_string(&config_map).ok()?;
        std::fs::write(path, config_json).ok()?;

        Some(true)
    }
}

fn default_daemon_config() -> DaemonConfig {
    let path: String = config_dir().unwrap().join("LibreHomework/librehomeworkd.log").to_str().unwrap().to_string();
    DaemonConfig {
        start_notification: Some(true),
        notify_on_error: Some(true),
        notify_on_warning: Some(true),
        remind_every: Some(30),
        remind_limit: Some(5),
        log_path: Some(path),
    }
}