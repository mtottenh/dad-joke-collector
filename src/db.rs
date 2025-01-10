use anyhow::Result;
use rusqlite::{Connection, params};
use crate::models::DadJoke;
use rand::Rng;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS jokes (
                id TEXT PRIMARY KEY,
                joke TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn insert_joke(&self, joke: &DadJoke) -> Result<bool> {
        let exists: bool = self
            .conn
            .query_row(
                "SELECT 1 FROM jokes WHERE id = ?",
                params![joke.id],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !exists {
            self.conn.execute(
                "INSERT INTO jokes (id, joke) VALUES (?, ?)",
                params![joke.id, joke.joke],
            )?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_random_joke(&self) -> Result<Option<String>> {
        // Get total count
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM jokes",
            [],
            |row| row.get(0),
        )?;

        if count == 0 {
            return Ok(None);
        }

        let random_index = rand::thread_rng().gen_range(0..count);
        
        let joke: Option<String> = self.conn.query_row(
            "SELECT joke FROM jokes LIMIT 1 OFFSET ?",
            params![random_index],
            |row| row.get(0),
        ).ok();

        Ok(joke)
    }
}
