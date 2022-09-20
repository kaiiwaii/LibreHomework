use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::errors::DBError;

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
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub documents: Vec<Document>,
}
impl Exam {
    pub fn from_row(value: &rusqlite::Row<'_>) -> Result<Self, Box<dyn Error>> {
        Ok(Exam {
            id: value.get(0)?,
            title: value.get(1)?,
            description: value.get(2)?,
            documents: Vec::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub id: Option<i32>,
    pub path: String,
    pub filename: String,
    pub description: String,
}
impl Document {
    pub fn from_row(value: &rusqlite::Row<'_>) -> Result<Self, DBError> {
        Ok(Document {
            id: value.get(0).map_err(|_| DBError::FetchError)?,
            path: value.get(1).map_err(|_| DBError::FetchError)?,
            filename: value.get(2).map_err(|_| DBError::FetchError)?,
            description: value.get(3).map_err(|_| DBError::FetchError)?,
        })
    }
}
