use rusqlite::Connection;
use std::sync::Mutex;
use crate::utils::stop_app;

#[derive(Debug)]
pub struct DBManager(pub Mutex<Connection>);

impl DBManager {
    pub fn prepare_connection(path: &std::path::Path) -> DBManager {
      let conn = Connection::open(path).unwrap_or_else(|_| stop_app("Error connecting to the database"));

      conn.execute("CREATE TABLE IF NOT EXISTS Subjects (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        name TEXT UNIQUE NOT NULL)", [])
        .unwrap_or_else(|_| stop_app("Error creating table Subjects"));

      conn.execute("CREATE TABLE IF NOT EXISTS Tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        name TEXT NOT NULL, subject TEXT NOT NULL, 
        description TEXT, 
        expires_at DATETIME NOT NULL, 
        FOREIGN KEY(subject) REFERENCES Subjects(name));", [])
        .unwrap_or_else(|_| stop_app("Error creating table Tasks"));

      conn.execute("CREATE TABLE IF NOT EXISTS Documents (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        path TEXT NOT NULL,
        filename TEXT NOT NULL,
        description TEXT);", [])
        .unwrap_or_else(|_| stop_app("Error creating table Documents"));

      conn.execute("CREATE TABLE IF NOT EXISTS Exams (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        title TEXT NOT NULL, description TEXT, 
        documents TEXT, 
        FOREIGN KEY(documents) REFERENCES Documents(id));", []).unwrap_or_else(|_| stop_app("Error creating table Exams"));
      return DBManager(Mutex::new(conn));
    }
}