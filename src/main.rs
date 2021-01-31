mod db;
mod runner;
use std::env;
use colored::*;

fn main() {
    let key = "GOGODB_PATH";

    let db = match env::var(key) {
        Ok(value) => db::open(value.as_str()),
        Err(_) => do_default()
    };

    runner::run(&db)
}

fn do_default() -> rocksdb::DB {
    println!("{}", "WARNING!!! Using /tmp/gogo.db".yellow().bold());
    println!("{}\n", "WARNING!!! Please do export GOGODB_PATH=/path/to/gogo.db ASAP!!!".yellow().bold());
    db::open("/tmp/gogo.db")
}
