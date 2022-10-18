use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_trim::string_trim;
use tabled::Tabled;

#[derive(Debug, Tabled, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Mnemonic {
    #[serde(deserialize_with = "string_trim")]
    pub key: String,
    #[serde(deserialize_with = "string_trim")]
    pub val: String,
}

/// Create the database
pub fn open(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mnemonic (
            key TEXT PRIMARY KEY,
            val TEXT NOT NULL
        )",
        (),
    )?;

    Ok(conn)
}

/// Get value from key
pub fn get(conn: &Connection, key: &str) -> Result<String> {
    let mut stmt = conn.prepare("SELECT val FROM mnemonic where key = ?")?;
    let value = stmt.query_row([key], |row| row.get::<_, String>(0))?;
    Ok(value)
}

/// Remove mnemonic => url mapping
pub fn remove(conn: &Connection, key: &str) -> Result<()> {
    let mut stmt = conn.prepare("DELETE FROM mnemonic WHERE key = ?")?;
    stmt.execute([key])?;
    Ok(())
}

/// Add key val pair to db
pub fn insert(conn: &Connection, key: &str, val: &str) -> Result<()> {
    conn.execute(
        "REPLACE INTO mnemonic (key, val) VALUES (?1, ?2)",
        (key, val),
    )?;
    Ok(())
}

/// List all key-vals from db
pub fn list_all(conn: &Connection) -> Result<Vec<Mnemonic>> {
    let mut stmt = conn.prepare("SELECT key, val FROM mnemonic")?;
    let mnemonic_iter = stmt.query_map([], |row| {
        Ok(Mnemonic {
            key: row.get(0)?,
            val: row.get(1)?,
        })
    })?;
    let mut mnemonics = vec![];
    for mnemonic in mnemonic_iter {
        let mnemonic = mnemonic?;
        if mnemonic.key != "_browser" {
            mnemonics.push(mnemonic)
        }
    }
    Ok(mnemonics)
}
