use log_panics;
use env_logger;

use std::thread::sleep;
use std::time::Duration;

mod db;
mod config;
mod notify;


fn main() {
    log_panics::init();
    env_logger::init();

    let config = config::DaemonConfig::read_config().unwrap();
    let mut db = db::Database::new().unwrap();

    if config.start_notification == Some(true) {
        notify::notify(notify::LogType::Info, "LibreHomework daemon started");
    }

    loop {
        let mut tasks = db.check().unwrap();
        tasks.sort_by(|t1, t2| t1.expires_at.cmp(&t2.expires_at));
        let mut c = 0;

        while c < config.remind_limit.unwrap() && c < tasks.len() as u32 {
            let task = tasks.get(c as usize).unwrap();

            if task.expires_at < std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() {
                notify::notify(notify::LogType::Warning, &format!("Task {} is expired", task.name));

            } else {
                notify::notify(notify::LogType::Info, &format!("Task {} expires in {} minutes", task.name, (task.expires_at - std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()) / 60));
            }
            c += 1;
        }

        sleep(Duration::from_secs(config.remind_every.unwrap() as u64 * 60));
    }
}
