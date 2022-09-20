use crate::errors::DBError;
use crate::models::{Document, Exam};
use crate::DBManager;
use rusqlite::params;
use tauri::State;

// #[tauri::command]
// pub fn remove_document(db: State<DBManager>, id: i32) -> Result<(), DBError> {

//     db.0.lock().map_err(|_| DBError::MutexError)?
//     .execute("DELETE FROM Documents WHERE id = ?1", params![id])
//     .map_err(|_| DBError::RemoveError)?;

//     Ok(())
// }

#[tauri::command]
pub fn get_exams(db: State<DBManager>) -> Result<Vec<Exam>, DBError> {
    let conn = db.0.lock().map_err(|_| DBError::MutexError)?;

    let mut query = conn
        .prepare("SELECT id, name, description, subject_id FROM Exams")
        .map_err(|_| DBError::PrepareError)?;
    let mut rows = query.query([]).map_err(|_| DBError::FetchError)?;

    let mut exams: Vec<Exam> = Vec::new();

    while let Some(row) = rows.next().map_err(|_| DBError::IteratingOverDataError)? {
        let mut exam = Exam::from_row(row).map_err(|_| DBError::ParsingModelError)?;

        let mut query = conn
            .prepare(
                "SELECT id, path, filename, description from Documents where attached_to_exam = ?1",
            )
            .map_err(|_| DBError::PrepareError)?;
        let mut rows = query.query([exam.id]).map_err(|_| DBError::FetchError)?;

        while let Some(row) = rows.next().map_err(|_| DBError::IteratingOverDataError)? {
            let doc = Document::from_row(row).map_err(|_| DBError::ParsingModelError)?;
            exam.documents.push(doc)
        }

        exams.push(exam);
    }

    Ok(exams)
}

#[tauri::command]
pub fn add_exam(
    db: State<DBManager>,
    name: &str,
    description: Option<String>,
    subject_id: i32,
    documents: Option<Vec<Document>>,
) -> Result<(), DBError> {
    let db = db.0.lock().map_err(|_| DBError::MutexError)?;
    db.execute(
        "INSERT INTO Exams (name, description, subject_id) VALUES (?1, ?2, ?3)",
        params![name, description, subject_id],
    )
    .map_err(|_| DBError::InsertError)?;

    // let id: u32 = db.query_row("SELECT id from Exams where name = ?!", [name], |r| r.get(0) )
    // .map_err(|_| DBError::FetchError)?;

    if documents.is_some() {
        for doc in documents.unwrap() {
            db.execute(
                "INSERT INTO Documents (path, filename, description, attached_to_exam) 
            VALUES (?1, ?2, ?3, (SELECT id FROM Exams where name = ?4)",
                [doc.path, doc.filename, doc.description, name.to_string()],
            )
            .map_err(|_| DBError::InsertError)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn remove_exam(db: State<DBManager>, id: i32) -> Result<(), DBError> {
    let db = db.0.lock().map_err(|_| DBError::MutexError)?;
    db.execute("DELETE FROM Exams WHERE id = ?1", params![id])
        .map_err(|_| DBError::RemoveError)?;

    db.execute(
        "DELETE FROM Documents WHERE attached_to_exam = ?1",
        params![id],
    )
    .map_err(|_| DBError::RemoveError)?;

    Ok(())
}
