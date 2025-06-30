use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Result as RusqliteResult};
use tokio::task;
use crate::errors::db_error::DbError;

#[derive(Clone)]
pub(crate) struct DbClient {
    con: Arc<Mutex<Connection>>,
}

impl DbClient {
    pub(crate) async fn new(db_path: &str) -> Result<Self, DbError> {
        let path = db_path.to_string();
        
        let con = task::spawn_blocking(move || -> RusqliteResult<Connection> {
            let con = Connection::open(&path)?;

            con.execute(
                "CREATE TABLE IF NOT EXISTS words (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        word_lowercase VARCHAR(50) NOT NULL,
                        word_pretty VARCHAR(50) NOT NULL)",
                [])?;

            con.execute(
                "CREATE TABLE IF NOT EXISTS word_chaining (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        word_id INTEGER,
                        next_word_id INTEGER,
                        guild_id BIGINT,
                        FOREIGN KEY(word_id) REFERENCES words(id),
                        FOREIGN KEY(next_word_id) REFERENCES words(id))",
                [])?;
            
            Ok(con)
        })
            .await?
            ?;
        
        Ok(DbClient { con: Arc::new(Mutex::new(con)) })
    }
}