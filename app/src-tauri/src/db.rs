use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use tauri::State;
use crate::utils::stop_app;


#[derive(Debug)]
pub struct DBManager(Mutex<Connection>);


//try with sqlite3 instead of rusqlite
impl DBManager {
    pub fn prepare_connection(path: &std::path::Path) -> DBManager {
      let conn = Connection::open(path).unwrap_or_else(|_| stop_app("Error connecting to the database"));

      conn.execute("CREATE TABLE IF NOT EXISTS Subjects (name TEXT NOT NULL)", []).unwrap_or_else(|_| stop_app("Error creating table Subjects"));

      conn.execute("CREATE TABLE IF NOT EXISTS Tasks (name TEXT NOT NULL, subject TEXT NOT NULL, description TEXT, expires_at DATETIME, 
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
pub fn execute(db: State<DBManager>, query: String) {
  db.0.lock().unwrap_or_else(|_| stop_app("Failed to access the database (unlocking mutex)"))
  .execute(&query, []).expect("Error executing query");
}