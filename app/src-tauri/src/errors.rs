use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum DBError {
    #[error("failed to unlock mutex")]
    MutexError,
    #[error("failed to insert data into the db")]
    InsertError,
    #[error("failed to remove row from db")]
    RemoveError,
    #[error("failed to fetch data from the db")]
    FetchError,
    #[error("failed to prepare the query")]
    PrepareError,
    #[error("failed to setup and prepare queries for the db")]
    SetupError,
    #[error("failed to parse date: invalid unix timestamp")]
    InvalidDataError,
    #[error("failed to iterate and parse data")]
    IteratingOverDataError,
    #[error("failed to parse db data into valid model")]
    ParsingModelError,
}

#[derive(Debug, Error)]
pub enum BootError {
    #[error("LibreHomework can't start: failed to detect config directory")]
    ConfigDirError,
    #[error("LibreHomework can't start: failed to create LibreHomework's config directory. Please check the path or the permissions")]
    CreateAppDirError,
}

#[derive(Debug, Error, Serialize)]
pub enum ConfigError {
    #[error("failed to read config file")]
    ReadError,
    #[error("failed to write config file")]
    WriteError,
}

#[derive(Debug, Error, Serialize)]
pub enum NetworkError {
    #[error("failed to read api response as json")]
    JsonError,
    #[error("failed to send request: {0}")]
    RequestError(String),
}
