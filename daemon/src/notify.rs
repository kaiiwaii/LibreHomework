use notify_rust::{Notification, Urgency};

pub enum LogType {
    Info,
    Warning,
    Error,
}

pub fn notify(log_type: LogType, message: &str) -> Option<()> {
    match log_type {
        LogType::Info => {
            Notification::new()
                .summary("LibreHomeworkd")
                .body(message)
                .icon("dialog-information")
                .urgency(Urgency::Low)
                .show().ok();
        },
        LogType::Warning => {
            Notification::new()
                .summary("LibreHomeworkd")
                .body(message)
                .icon("dialog-warning")
                .urgency(Urgency::Normal)
                .show().ok();
        },
        LogType::Error => {
            Notification::new()
                .summary("LibreHomeworkd")
                .body(message)
                .icon("dialog-error")
                .urgency(Urgency::Critical)
                .show().ok();
        },
    }
    Some(())
}