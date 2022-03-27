use log_panics;
use env_logger;

use std::thread::sleep;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

mod db;
mod config;
mod notify;
use notify::{notify, LogType};


fn main() {
    log_panics::init();
    env_logger::init();

    let config = if let Some(cfg) = config::DaemonConfig::read_config() {
        cfg
    } else {
        notify(LogType::Error, "LibreHomework Daemon: Failed to read config file");
        panic!("Failed to read config file");
    };

    let mut db = if let Some(db) = db::Database::new() {
        db
    } else {
        notify(LogType::Error, "LibreHomework Daemon: Failed to open database");
        panic!("Failed to open database");
    };

    if config.start_notification == Some(true) {
        notify(LogType::Info, "LibreHomework Daemon started");
    }

    loop {
        let mut tasks = if let Ok(data) = db.check() {
            data
        } else {
            notify(LogType::Error, "LibreHomework Daemon: Failed to read database");
            panic!("Failed to get tasks from database");
        };
        
        tasks.sort_by(|t1, t2| t1.expires_at.cmp(&t2.expires_at));

        let mut c = 0;

        while c < config.remind_limit.unwrap() && c < tasks.len() as u32 {
            let task = tasks.get(c as usize).unwrap();

            if task.expires_at < std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() {
                notify(LogType::Warning, &format!("Task {} is expired", task.name));

            } else {
                notify(LogType::Info, &format!("Task {} expires in {}", task.name, choose_time(task.expires_at - SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()) ));
            }
            c += 1;
        }

        if c < tasks.len() as u32 {
            notify::notify(notify::LogType::Warning, &format!("More tasks not notified: {}", tasks.len() - c as usize));
        }


        sleep(Duration::from_secs(config.remind_every.unwrap() as u64 * 60));
    }
}

pub fn choose_time(secs: u64) -> String {
    if secs < 60 {
        return format!("{} seconds", secs);
    } else if secs < 60 * 60 {
        return format!("{} minutes", secs / 60);
    } else if secs < 60 * 60 * 24 {
        return format!("{} hours", secs / (60 * 60));
    } else {
        return format!("{} days", secs / (60 * 60 * 24));
    }
}
