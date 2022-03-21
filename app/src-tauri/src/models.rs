use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u32,
    name: String,
    subject: String,
    description: Option<String>,
    expires_at: i64,
}
impl Task {
    pub fn from_row(value: &rusqlite::Row<'_>) -> Result<Self, Box<dyn Error>> {
        Ok(Task {
            id: value.get(0)?,
            name: value.get(1)?,
            subject: value.get(2)?,
            description: value.get(3)?,
            expires_at: value.get(4)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    id: i32,
    name: String,
}
impl Subject {
    pub fn from_row(value: &rusqlite::Row<'_>) -> Result<Self, Box<dyn Error>> {
        Ok(Subject {
            id: value.get(0)?,
            name: value.get(1)?,
        })
    }
}