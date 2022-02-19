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
        let tasks = db.check().unwrap();
        for task in tasks {
            notify::notify(notify::LogType::Info, &("Remember to do: ".to_string() + &task.name));
        }
        sleep(Duration::from_secs(config.remind_every.unwrap() as u64 * 60));
    }
}
