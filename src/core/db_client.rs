use std::sync::{Arc, Mutex};
use log::error;
use rusqlite::{Connection, OptionalExtension, Result as RusqliteResult};
use tokio::task;
use crate::errors::db_error::DbError;
use crate::storage::app_properties_model::Db;
use crate::storage::db_model::GuildModel;

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
                "CREATE TABLE IF NOT EXISTS guilds (
                id BIGINT PRIMARY KEY,
                name VARCHAR(50) NOT NULL UNIQUE,
                system_channel_id BIGINT)",
                [])?;

            con.execute(
                "CREATE TABLE IF NOT EXISTS word_chaining (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        word_id INTEGER,
                        next_word_id INTEGER,
                        frequency INTEGER,
                        guild_id BIGINT,
                        FOREIGN KEY(guild_id) REFERENCES guilds(id),
                        FOREIGN KEY(word_id) REFERENCES words(id),
                        FOREIGN KEY(next_word_id) REFERENCES words(id))",
                [])?;

            Ok(con)
        }).await??;
        
        Ok(DbClient { con: Arc::new(Mutex::new(con)) })
    }
    
    pub(crate) async fn store_word(&self, word: String) -> RusqliteResult<(), DbError> {
        let con_arc = Arc::clone(&self.con);
        
        task::spawn_blocking(move || {
            let con_guard = con_arc
                .lock()
                .unwrap();

            let dup_check: Option<i32> = con_guard.query_one(
                "SELECT w.id FROM words w WHERE w.word_lowercase = ?1",
                [word.to_lowercase()],
                |row| Ok(row.get(0)?)
            ).ok();

            if let None = dup_check {
                con_guard.execute(
                    "INSERT INTO words (word_lowercase, word_pretty) VALUES (?1, ?2)",
                    [word.to_lowercase(), word])?;
            }

            Ok(())
        }).await?
    }
    
    pub(crate) async fn store_word_chaining(&self, guild_id: u64, current_word: String, next_word: String) -> RusqliteResult<(), DbError> {
        let con_arc = Arc::clone(&self.con);
        
        task::spawn_blocking(move || {
            let con_guard = con_arc
                .lock()
                .unwrap();

            let cur_word_id: Option<i32> = con_guard.query_one(
                "SELECT w.id FROM words w WHERE w.word_lowercase = ?1",
                [current_word.to_lowercase()],
                |row| Ok(row.get(0)?))?;


            let next_word_id: Option<i32> = con_guard.query_one(
                "SELECT w.id FROM words w WHERE w.word_lowercase = ?1",
                [next_word.to_lowercase()],
                |row| Ok(row.get(0)?))
                .ok();

            let dupe_check: Option<i32> = con_guard
                .query_one("SELECT id FROM word_chaining wc WHERE wc.word_id = ?1 AND wc.next_word_id = ?2 AND wc.guild_id = ?3",
                           (cur_word_id.unwrap(), next_word_id.unwrap_or(287), guild_id.clone()),
                           |row| Ok(row.get(0)?))
                .ok();

            // TODO: Code the possibility of lone word case
            match dupe_check {
                None => {
                    if cur_word_id.is_some() && next_word_id.is_some() {
                        con_guard.execute(
                            "INSERT INTO word_chaining (word_id, next_word_id, frequency, guild_id) VALUES (?1, ?2, ?3, ?4)"
                            , (cur_word_id.unwrap(), next_word_id.unwrap(), 1, guild_id))?;
                    }
                },
                Some(id) => {
                    con_guard
                        .execute("UPDATE word_chaining SET frequency = frequency + 1 WHERE id = ?1",
                                 [id])?;
                }
            };

            Ok(())
        }).await?
    }

    pub(crate) async fn store_guild(&self, guild_id: u64, guild_name: String, system_channel_id: Option<u64>) -> RusqliteResult<(), DbError> {
        let con_arc = Arc::clone(&self.con);

        task::spawn_blocking(move || {
            let con_guard = con_arc
                .lock()
                .unwrap();

            con_guard.execute(
                "INSERT INTO guilds (id, name, system_channel_id) VALUES (?1, ?2, ?3)",
                (guild_id, guild_name, system_channel_id))?;

            Ok(())
        }).await?
    }

    pub(crate) async fn is_guild_new(&self, guild_id: u64) -> RusqliteResult<Option<GuildModel>, DbError> {
        let con_arc = Arc::clone(&self.con);

        task::spawn_blocking(move || {
            let con_guard = con_arc
                .lock()
                .unwrap();

            let check_guild = con_guard
                .query_one("SELECT * FROM guilds g WHERE g.id = ?1", [guild_id], |row| Ok(GuildModel {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    system_channel_id: row.get(2)?,
                })).optional();

            return if let Some(guild) = check_guild? {
                Ok(Some(guild))
            } else {
                Ok(None)
            }
        }).await?
    }

    pub(crate) async fn count_words(&self, guild_id: u64) -> RusqliteResult<Option<usize>, DbError> {
        let con_arc = Arc::clone(&self.con);
        task::spawn_blocking(move || {
            let con_guard = con_arc
                .lock()
                .unwrap();

            let count: Option<usize> = con_guard.query_one(
                "SELECT COUNT(DISTINCT words.word_id)
                FROM (SELECT word_id AS word_id FROM word_chaining wc WHERE wc.guild_id = ?1
                UNION
                SELECT next_word_id AS word_id FROM word_chaining wc WHERE wc.guild_id = ?1) words;",
                [guild_id],
                |row| Ok(row.get(0)?))
                .ok();

            Ok(count)
        }).await?


    }
}