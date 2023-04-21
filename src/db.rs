use anyhow::{bail, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_trim::string_trim;
use std::{
    fs::{copy, remove_file},
    path::Path,
};
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

/// Create the v2 database (with id as the primary key)
pub fn open_v2(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mnemonic (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT UNIQUE NOT NULL,
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
    let mut stmt = conn.prepare("REPLACE INTO mnemonic (key, val) VALUES (?1, ?2)")?;
    stmt.execute([key, val])?;
    Ok(())
}

/// List all key-vals from db
pub fn list_all(conn: &Connection) -> Result<Vec<Mnemonic>> {
    let mut stmt = match is_v2(conn)? {
        false => conn.prepare("SELECT key, val FROM mnemonic")?,
        true => conn.prepare("SELECT key, val FROM mnemonic order by id asc")?,
    };

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

pub fn migrate(conn: &Connection) -> Result<()> {
    println!("starting migration of old db to new db (in place)");
    match conn.path() {
        None => bail!("db path not found!"),
        Some(db_str) => {
            let db_path = Path::new(db_str);
            if let Some(parent) = db_path.parent() {
                if let Some(parent_str) = parent.to_str() {
                    let tmp_str = format!("{}/gogo_bak.sqlite", parent_str);
                    let backup_path = Path::new(&tmp_str);
                    copy(db_str, backup_path)?;
                    println!("copying old db to tmp path: {:?}", backup_path);
                    remove_file(db_path)?;
                    println!("removing old db {:?}", db_path);
                    println!("creating new empty database {:?}", db_str);
                    let new_conn = open_v2(db_str)?;
                    println!("populating new database {:?}", db_str);
                    populate_v2(&Connection::open(backup_path)?, &new_conn)?;
                    println!("removing tmp database {:?}", backup_path);
                    remove_file(backup_path)?;
                } else {
                    bail!("malformed parent dir str")
                }
            } else {
                bail!("no parent dir!")
            }
        }
    }
    Ok(())
}

fn populate_v2(old_conn: &Connection, new_conn: &Connection) -> Result<()> {
    let mnemonics = list_all(old_conn)?;

    for mnemonic in mnemonics {
        new_conn.execute(
            "INSERT INTO mnemonic (key, val) VALUES (?1, ?2)",
            [&mnemonic.key, &mnemonic.val],
        )?;
    }

    Ok(())
}

fn get_primary_key(conn: &Connection, table_name: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
    let rows = stmt.query_map([], |row| {
        let column_name: String = row.get(1)?;
        let is_pk: u32 = row.get(5)?;
        Ok((column_name, is_pk))
    })?;

    for row in rows {
        let (column_name, is_pk) = row?;
        if is_pk == 1 {
            return Ok(Some(column_name));
        }
    }
    Ok(None)
}

fn is_v2(conn: &Connection) -> Result<bool> {
    match get_primary_key(conn, "mnemonic")? {
        Some(pk) => {
            if pk == *"id".to_string() {
                return Ok(true);
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let test_db_path = "/tmp/gogotest.db";
        let conn = open(test_db_path).expect("failed to create gogotest database");
        insert(&conn, "key", "value").expect("unable to insert key:val");
        insert(&conn, "key2", "value2").expect("unable to insert key:val");
        let val = get(&conn, "key").expect("unable to get value");
        assert_eq!("value", val);
        let mnemonics = list_all(&conn).expect("unable to list mnemonics");
        assert!(mnemonics.len() == 2);
        std::fs::remove_file(test_db_path).expect("unable to clean up gogotest database");
    }
}
