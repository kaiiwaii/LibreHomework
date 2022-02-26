use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use tauri::State;
use crate::utils::stop_app;
use crate::models::*;


#[derive(Debug)]
pub struct DBManager(Mutex<Connection>);

impl DBManager {
    pub fn prepare_connection(path: &std::path::Path) -> DBManager {
      let conn = Connection::open(path).unwrap_or_else(|_| stop_app("Error connecting to the database"));

      conn.execute("CREATE TABLE IF NOT EXISTS Subjects (name TEXT NOT NULL)", []).unwrap_or_else(|_| stop_app("Error creating table Subjects"));

      conn.execute("CREATE TABLE IF NOT EXISTS Tasks (name TEXT NOT NULL, subject TEXT NOT NULL, description TEXT, expires_at DATETIME NOT NULL, 
      FOREIGN KEY(subject) REFERENCES Subjects(name));", []).unwrap_or_else(|_| stop_app("Error creating table Tasks"));
      return DBManager(Mutex::new(conn));
    }
}

#[tauri::command]
pub fn addTask(db: State<DBManager>, name: &str, subject: &str, description: &str, expires_at: &str) -> Result<(), String> {

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
pub fn removeTask(db: State<DBManager>, name: &str) -> Result<(), String> {

  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute("DELETE FROM Tasks WHERE name = ?1", params![name]).unwrap_or_else(|_| stop_app("Error removing task")); //maybe too agressive?

   Ok(())
}

#[tauri::command]
pub fn getTasks(db: State<DBManager>, limit: u32, page: u32) -> Result<Vec<Task>, String> {

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
pub fn getSubjects(db: State<DBManager>) -> Option<Vec<String>> {
  let conn = db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"));
  let mut query = conn.prepare("SELECT name FROM Subjects").unwrap_or_else(|_| stop_app("Error preparing query"));//maybe too agressive?
  let mut rows = query.query([]).unwrap_or_else(|_| stop_app("Error querying"));
  let mut vec: Vec<String> = Vec::new();
  while let Some(row) = rows.next().unwrap_or_else(|_| stop_app("Error iterating")) {
    vec.push(row.get(0).unwrap_or_else(|_| stop_app("Error parsing row")));
  }
  Some(vec)
}

#[tauri::command]
pub fn addSubject(db: State<DBManager>, name: &str) -> Option<bool> {

  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute("INSERT INTO Subjects (name) VALUES (?1)", params![name]).ok()?;

  Some(true)
}

#[tauri::command]
pub fn execute(db: State<DBManager>, query: String) {
  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute(&query, []).expect("Error executing query");
}