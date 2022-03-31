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

#[derive(Serialize, Deserialize, Debug)]
pub struct Exam {
    id: i32,
    title: String,
    description: Option<String>,
    documents: Option<Vec<String>>
}
impl Exam {
    pub fn from_row(value: &rusqlite::Row<'_>) -> Result<Self, Box<dyn Error>> {
        let documents: String = value.get(3)?;
        Ok(Exam {
            id: value.get(0)?,
            title: value.get(1)?,
            description: value.get(2)?,
            documents: Some(documents.split(",").map(|s| s.to_string()).collect()),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    id: i32,
    path: String,
    filename: String,
    description: Option<String>,
}
impl Document {
    pub fn from_row(value: &rusqlite::Row<'_>) -> Result<Self, Box<dyn Error>> {
        Ok(Document {
            id: value.get(0)?,
            path: value.get(1)?,
            filename: value.get(2)?,
            description: value.get(3)?,
        })
    }
}