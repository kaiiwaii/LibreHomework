use crate::errors::DBError;
use crate::models::*;
use crate::DBManager;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn add_task(
    db: State<DBManager>,
    name: &str,
    subject: &str,
    description: &str,
    expires_at: &str,
    docs: Vec<Document>,
) -> Result<(), DBError> {
    let db = db.0.lock().map_err(|_| DBError::MutexError)?;

    let expiration_date_parsed = match expires_at.parse::<i64>() {
        Ok(date) => date,
        Err(_) => return Err(DBError::InvalidDataError),
    };

    db.execute(
        "INSERT INTO Tasks (
    name, subject, description, expires_at) 
    VALUES (?1, ?2, ?3, ?4)",
        params![name, subject, description, expiration_date_parsed],
    )
    .map_err(|_| DBError::InsertError)?;

    for doc in docs {
        db.execute(
            "INSERT INTO Documents (path, filename, description) VALUES (?1, ?2, ?3);",
            params![doc.path, doc.filename, doc.description],
        )
        .map_err(|_| DBError::InsertError)?;
    }

    Ok(())
}

#[tauri::command]
pub fn remove_task(db: State<DBManager>, id: u32) -> Result<(), DBError> {
    let db = db.0.lock().map_err(|_| DBError::MutexError)?;

    db.execute("DELETE FROM Tasks WHERE id = ?1", params![id])
        .map_err(|_| DBError::RemoveError)?;

    Ok(())
}

#[tauri::command]
pub fn get_tasks(db: State<DBManager>, limit: u32, page: u32) -> Result<Vec<Task>, DBError> {
    let mut vec: Vec<Task> = Vec::new();

    let conn = db.0.lock().map_err(|_| DBError::MutexError)?;

    let mut query = conn
        .prepare("SELECT * FROM Tasks LIMIT ?1 OFFSET ?2")
        .map_err(|_| DBError::PrepareError)?;
    let mut rows = query
        .query(params![limit, page * 20])
        .map_err(|_| DBError::FetchError)?;

    while let Some(row) = rows.next().map_err(|_| DBError::IteratingOverDataError)? {
        vec.push(Task::from_row(&row).map_err(|_| DBError::ParsingModelError)?);
    }

    Ok(vec)
}

#[tauri::command]
pub fn get_task_documents(db: State<DBManager>, id: u32) -> Result<Vec<Document>, DBError> {
    let conn = db.0.lock().map_err(|_| DBError::MutexError)?;
    let mut docs: Vec<Document> = Vec::new();

    let mut query = conn
        .prepare(
            "SELECT id, path, filename, description from Documents where attached_to_task = ?1",
        )
        .map_err(|_| DBError::PrepareError)?;
    let mut rows = query.query([id]).map_err(|_| DBError::FetchError)?;

    while let Some(row) = rows.next().map_err(|_| DBError::IteratingOverDataError)? {
        let doc = Document::from_row(row).map_err(|_| DBError::ParsingModelError)?;
        docs.push(doc)
    }
    Ok(docs)
}

#[tauri::command]
pub fn get_subjects(db: State<DBManager>) -> Result<Vec<Subject>, DBError> {
    let conn = db.0.lock().map_err(|_| DBError::MutexError)?;

    let mut query = conn
        .prepare("SELECT id, name FROM Subjects")
        .map_err(|_| DBError::PrepareError)?;
    let mut rows = query.query([]).map_err(|_| DBError::FetchError)?;
    let mut vec: Vec<Subject> = Vec::new();

    while let Some(row) = rows.next().map_err(|_| DBError::IteratingOverDataError)? {
        vec.push(Subject::from_row(row).map_err(|_| DBError::ParsingModelError)?);
    }
    Ok(vec)
}

#[tauri::command]
pub fn add_subject(db: State<DBManager>, name: &str) -> Result<(), DBError> {
    db.0.lock()
        .map_err(|_| DBError::MutexError)?
        .execute("INSERT INTO Subjects (name) VALUES (?1)", params![name])
        .map_err(|_| DBError::InsertError)?;

    Ok(())
}

#[tauri::command]
pub fn remove_subject(db: State<DBManager>, id: i32) -> Result<(), DBError> {
    db.0.lock()
        .map_err(|_| DBError::MutexError)?
        .execute("DELETE FROM Subjects WHERE id = ?1", params![id])
        .map_err(|_| DBError::RemoveError)?;

    Ok(())
}
