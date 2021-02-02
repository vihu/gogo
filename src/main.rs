mod db;
mod runner;
use colored::*;
use std::env;

// env db path key
const ENV_DB_PATH: &str = "GOGODB_PATH";
// default database path if ENV_DB_PATH is not set
const DEFAULT_DB_PATH: &str = "/tmp/gogo.db";
// default browser key whose value can be updated by a user if they want to switch browser via CLI
const BROWSER_KEY: &str = "_browser";
const BROWSER_VAL: &str = "firefox";

fn main() {
    let db = match env::var(ENV_DB_PATH) {
        Ok(value) => db::open(value.as_str()),
        Err(_) => do_default(),
    };

    runner::run(&db)
}

fn do_default() -> rocksdb::DB {
    println!("{}", "WARNING!!! Using /tmp/gogo.db".yellow().bold());
    println!(
        "{}\n",
        "WARNING!!! Please do export GOGODB_PATH=/path/to/gogo.db ASAP!!!"
            .yellow()
            .bold()
    );
    db::open(DEFAULT_DB_PATH)
}
