use crate::errors::DBError;
use rusqlite::Connection;
use std::sync::Mutex;

#[derive(Debug)]
pub struct DBManager(pub Mutex<Connection>);

impl DBManager {
    pub fn prepare_connection(path: &std::path::Path) -> Result<DBManager, DBError> {
        let conn = Connection::open(path).map_err(|_| DBError::SetupError)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Subjects (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        name TEXT UNIQUE NOT NULL)",
            [],
        )
        .map_err(|_| DBError::SetupError)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        name TEXT NOT NULL, subject TEXT NOT NULL, 
        description TEXT, 
        expires_at DATETIME NOT NULL, 
        FOREIGN KEY(subject) REFERENCES Subjects(name));",
            [],
        )
        .map_err(|_| DBError::SetupError)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Exams (
          id INTEGER PRIMARY KEY AUTOINCREMENT, 
          title TEXT NOT NULL,
          description TEXT,
          subject TEXT NOT NULL,
          FOREIGN KEY(subject) REFERENCES Subjects(name));
          ",
            [],
        )
        .map_err(|_| DBError::SetupError)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Documents (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        path TEXT NOT NULL,
        filename TEXT NOT NULL,
        description TEXT,
        attached_to_task INTEGER,
        attached_to_exam INTEGER,
        FOREIGN KEY (attached_to_task) REFERENCES Tasks(id)
        FOREIGN KEY (attached_to_exam) REFERENCES Exams(id));",
            [],
        )
        .map_err(|_| DBError::SetupError)?;

        return Ok(DBManager(Mutex::new(conn)));
    }
}
