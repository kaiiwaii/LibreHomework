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

            if task.expires_at < unix_now() {
                notify(LogType::Warning, &format!("Task {} is expired", task.name));

            } else {
                println!("{}", task.expires_at - unix_now());
                notify(LogType::Info, &format!("Task {} expires in {}", task.name, choose_time(task.expires_at)));
            }
            
            c += 1;
        }

        if c < tasks.len() as u32 {
            notify::notify(notify::LogType::Warning, &format!("More tasks not notified: {}", tasks.len() - c as usize));
        }


        sleep(Duration::from_secs(config.remind_every.unwrap() as u64 * 60));
    }
}


fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}


fn choose_time(expires_at: u64) -> String {
    println!("expires_at: {}", expires_at);
    let now = unix_now();
    println!("now: {}", now);
    let diff = expires_at - now;
    println!("diff: {}", diff);

    if diff < 60 {
        return "less than a minute".to_string();
    }

    if diff < 3600 {
        return format!("{} minutes", diff / 60);
    }

    if diff < 86400 {
        return format!("{} hours", diff / 3600);
    }

    if diff < 2592000 {
        return format!("{} days", diff / 86400);
    }

    format!("{} months", diff / 2592000)
    
}
