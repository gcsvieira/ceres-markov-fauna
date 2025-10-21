use std::ops::Add;
use std::sync::{Arc, Mutex};
use log::error;
use rusqlite::{Connection, Result as RusqliteResult};
use serenity::futures::StreamExt;
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
                        word_lowercase VARCHAR(50) NOT NULL UNIQUE,
                        word_pretty VARCHAR(50) NOT NULL)",
                [])?;

            con.execute(
                "CREATE TABLE IF NOT EXISTS word_chaining (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        word_id INTEGER,
                        next_word_id INTEGER,
                        frequency INTEGER,
                        guild_id BIGINT,
                        FOREIGN KEY(word_id) REFERENCES words(id),
                        FOREIGN KEY(next_word_id) REFERENCES words(id))",
                [])?;
            
            Ok(con)
        }).await?
            ?;
        
        Ok(DbClient { con: Arc::new(Mutex::new(con)) })
    }
    
    pub(crate) async fn store_word(&self, word: String) -> RusqliteResult<(), DbError> {
        let con_arc = Arc::clone(&self.con);
        
        task::spawn_blocking(move || {
            let con_guard = con_arc
                .lock()
                .unwrap();

            con_guard.execute(
                "INSERT INTO words (word_lowercase, word_pretty) VALUES (?, ?)",
                [word.to_lowercase(), word])?;
            
            Ok(())
        }).await?
    }
    
    pub(crate) async fn is_not_duplicate(&self, possible_duplicate: String) -> RusqliteResult<bool, DbError> {
        let con_arc = Arc::clone(&self.con);
        
        task::spawn_blocking(move || {
            let con_guard = con_arc
                .lock()
                .unwrap();
            
            let mut dup_check = con_guard.prepare(
                "SELECT w.word_lowercase FROM words w WHERE w.word_lowercase = ?"
            )?;

            let mut word = Vec::new();
            let word_iter = dup_check.query_map([possible_duplicate.to_lowercase()], |row| { row.get(0) })?;
            
            for word_result in word_iter {
                word.push(word_result?);
            }
            
            if word.is_empty() {
                Ok(false)
            } else {
                Ok(true)
            }
        }).await?
    }
    
    pub(crate) async fn store_word_chaining(&self, current_word: String, next_word: String) -> RusqliteResult<(), DbError> {
        let con_arc = Arc::clone(&self.con);
        
        task::spawn_blocking(move || {
            let con_guard = con_arc
                .lock()
                .unwrap();

            con_guard.execute(
                "INSERT INTO word_chaining ("
                , []).expect("TODO: panic message");
            
            Ok(())
        }).await?
    }
}