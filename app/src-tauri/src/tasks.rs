use crate::{DBManager};
use crate::utils::stop_app;
use crate::models::*;
use tauri::State;
use rusqlite::params;

#[tauri::command]
pub fn add_task(db: State<DBManager>, name: &str, subject: &str, description: &str, expires_at: &str) -> Result<(), String> {

  let expiration_date_parsed = match expires_at.parse::<i64>() {
    Ok(date) => date,
    Err(_) => {return Err("Invalid date".to_string())},
  };

  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute("INSERT INTO Tasks (
    name, subject, description, expires_at) VALUES (?1, ?2, ?3, ?4)",
   params![name, subject, description, expiration_date_parsed]).unwrap_or_else(|_| stop_app("Error inserting task")); //maybe too agressive?

   Ok(())
}

#[tauri::command]
pub fn remove_task(db: State<DBManager>, id: u32) -> Result<(), String> {

  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute("DELETE FROM Tasks WHERE id = ?1", params![id]).unwrap_or_else(|_| stop_app("Error removing task")); //maybe too agressive?

   Ok(())
}

#[tauri::command]
pub fn get_tasks(db: State<DBManager>, limit: u32, page: u32) -> Result<Vec<Task>, String> {

  let mut vec: Vec<Task> = Vec::new();

  let conn = db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"));
  let mut query = conn.prepare("SELECT * FROM Tasks LIMIT ?1 OFFSET ?2").unwrap_or_else(|_| stop_app("Error preparing query"));//maybe too agressive?
  let mut rows = query.query(params![limit, page * 20]).unwrap_or_else(|_| stop_app("Error querying"));
  while let Some(row) = rows.next().unwrap_or_else(|_| stop_app("Error iterating")) {
    vec.push(Task::from_row(&row).unwrap_or_else(|_| stop_app("Error parsing row")));
  }

  Ok(vec)
}

#[tauri::command]
pub fn get_subjects(db: State<DBManager>) -> Option<Vec<Subject>> {
  let conn = db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"));

  let mut query = conn.prepare("SELECT id, name FROM Subjects").unwrap_or_else(|_| stop_app("Error preparing query"));//maybe too agressive?
  let mut rows = query.query([]).unwrap_or_else(|_| stop_app("Error querying"));
  let mut vec: Vec<Subject> = Vec::new();
  
  while let Some(row) = rows.next().unwrap_or_else(|_| stop_app("Error iterating over subjects")) {
    vec.push(Subject::from_row(row).unwrap_or_else(|_| stop_app("Error parsing subject row")));
  }
  Some(vec)
}

#[tauri::command]
pub fn add_subject(db: State<DBManager>, name: &str) -> Option<bool> {

  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute("INSERT INTO Subjects (name) VALUES (?1)", params![name]).ok()?;

  Some(true)
}

#[tauri::command]
pub fn remove_subject(db: State<DBManager>, id: i32) -> Option<bool> {

  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute("DELETE FROM Subjects WHERE id = ?1", params![id]).ok()?;

  Some(true)
}

#[tauri::command]
pub fn execute(db: State<DBManager>, query: String) {
  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute(&query, []).expect("Error executing query");
}