use std::fmt;
use std::fmt::Formatter;
use rusqlite::Error as RusqliteError;
use tokio::task::JoinError;

#[derive(Debug)]
pub(crate) enum DbError {
    Rusqlite(RusqliteError),
    Join(JoinError),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Rusqlite(err) => write!(f, "A database related error occurred: {}", err),
            DbError::Join(err) => write!(f, "An internal task error occurred: {}", err)
        }
    }
}

impl std::error::Error for DbError {}

impl From<RusqliteError> for DbError {
    fn from(err: RusqliteError) -> Self {
        DbError::Rusqlite(err)
    }
}

impl From<JoinError> for DbError {
    fn from(err: JoinError) -> Self {
        DbError::Join(err)
    }
}