use crate::models::{Document, Exam};
use tauri::State;
use crate::utils::stop_app;
use rusqlite::params;
use crate::DBManager;

#[tauri::command]
pub fn get_documents(db: State<DBManager>, amount: u8) -> Option<Vec<Document>> {
    let conn = db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"));

    let mut query = conn.prepare("SELECT id, path, filename, description FROM Documents").unwrap_or_else(|_| stop_app("Error preparing query"));//maybe too agressive?
    let mut rows = query.query([]).unwrap_or_else(|_| stop_app("Error querying"));
    let mut vec: Vec<Document> = Vec::new();

    while let Some(row) = rows.next().unwrap_or_else(|_| stop_app("Error iterating over documents")) {
        vec.push(Document::from_row(row).unwrap_or_else(|_| stop_app("Error parsing document row")));
    }
    Some(vec)
}

#[tauri::command]
pub fn add_document(db: State<DBManager>, path: &str, filename: &str, description: Option<String>) -> Option<bool> {

    db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
    .execute("INSERT INTO Documents (path, filename, description) VALUES (?1, ?2, ?3)",
    params![path, filename, description]).ok()?;

    Some(true)
}

#[tauri::command]
pub fn remove_document(db: State<DBManager>, id: i32) -> Option<bool> {

    db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
    .execute("DELETE FROM Documents WHERE id = ?1", params![id]).ok()?;

    Some(true)
}

#[tauri::command]
pub fn get_exams(db: State<DBManager>, amount: u8) -> Option<Vec<Exam>> {
    let conn = db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"));

    let mut query = conn.prepare("SELECT id, name, description, subject_id FROM Exams").unwrap_or_else(|_| stop_app("Error preparing query"));//maybe too agressive?
    let mut rows = query.query([]).unwrap_or_else(|_| stop_app("Error querying"));
    let mut vec: Vec<Exam> = Vec::new();

    while let Some(row) = rows.next().unwrap_or_else(|_| stop_app("Error iterating over exams")) {
        vec.push(Exam::from_row(row).unwrap_or_else(|_| stop_app("Error parsing exam row")));
    }
    Some(vec)
}

#[tauri::command]
pub fn add_exam(db: State<DBManager>, name: &str, description: Option<String>, subject_id: i32) -> Option<bool> {

    db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
    .execute("INSERT INTO Exams (name, description, subject_id) VALUES (?1, ?2, ?3)",
    params![name, description, subject_id]).ok()?;

    Some(true)
}

#[tauri::command]
pub fn remove_exam(db: State<DBManager>, id: i32) -> Option<bool> {

    db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
    .execute("DELETE FROM Exams WHERE id = ?1", params![id]).ok()?;

    Some(true)
}
