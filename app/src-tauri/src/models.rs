use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    name: String,
    subject: String,
    description: Option<String>,
    expires_at: i64,
}
impl Task {
    pub fn from_row(value: &rusqlite::Row<'_>) -> Result<Self, Box<dyn Error>> {
        Ok(Task {
            name: value.get(0)?,
            subject: value.get(1)?,
            description: value.get(2)?,
            expires_at: value.get(3)?,
        })
    }
}