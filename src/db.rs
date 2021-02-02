use crate::{BROWSER_KEY, BROWSER_VAL};
use colored::*;
use prettytable::{cell, row, Table};
use rocksdb::{IteratorMode, DB};

// create the database
pub fn open(path: &str) -> DB {
    let db = DB::open_default(path).unwrap();
    // maybe insert default browser key/val pair if it doesn't exist
    maybe_insert(&db, BROWSER_KEY, BROWSER_VAL);
    db
}

// list all mnemonic mappings
pub fn list_mnemonics(db: &DB) {
    let iter = db.iterator(IteratorMode::Start);

    let mut table = Table::new();

    table.add_row(row!["Mnemonic".yellow().bold(), "URL".yellow().bold()]);

    for (key, value) in iter {
        let human_key = String::from_utf8(key.to_vec()).unwrap();
        if human_key != BROWSER_KEY {
            let human_value = String::from_utf8(value.to_vec()).unwrap();
            table.add_row(row![human_key.cyan(), human_value.green()]);
        }
    }

    table.printstd();
}

// get set browser
pub fn get_browser(db: &DB) -> Option<String> {
    match db.get(&BROWSER_KEY) {
        Ok(Some(browser)) => {
            let actual_browser = String::from_utf8(browser).unwrap();
            Some(actual_browser)
        }
        Ok(None) => {
            println!("{}", "No browser configured".yellow().bold());
            None
        }
        Err(_) => {
            println!("{}", "Unable to get browser".red().bold());
            None
        }
    }
}

// get url from mnemonic
pub fn get_url_from_mnemonic(db: &DB, key: &str) -> Option<String> {
    match db.get(key) {
        Ok(Some(url)) => Some(String::from_utf8(url).unwrap()),
        Ok(None) => None,
        Err(_) => None,
    }
}

// remove mnemonic => url mapping
pub fn remove(db: &DB, key: &str) {
    match db.get(key) {
        Ok(None) => println!("{}", "key does not exist".yellow()),
        Ok(_) => match db.delete(key) {
            Ok(()) => {
                println!("{} removed", key.green());
                println!("{}", "Updated list".purple().bold());
                list_mnemonics(db)
            }
            Err(_) => println!("{}", "Unable to remove key".red().bold()),
        },
        Err(_) => println!("{}", "Unable to remove key".red().bold()),
    }
}

// add key val pair to db
pub fn insert(db: &DB, key: &str, val: &str) {
    match db.put(key, val) {
        Ok(_) => insert_help(key, val),
        Err(_) => insert_help(key, val),
    }
}

fn insert_help(key: &str, value: &str) {
    println!("key: {} added, value: {}", key.green(), value.green())
}

// add key/val if it does not exist already
pub fn maybe_insert(db: &DB, key: &str, val: &str) {
    match db.get(key) {
        Ok(None) => match db.put(key, val) {
            Ok(()) => insert_help(key, val),
            Err(_) => println!("{}", "Unable to remove key".red().bold()),
        },
        Ok(_) => (),
        Err(_) => println!("{}", "Unable to insert key".red().bold()),
    }
}
