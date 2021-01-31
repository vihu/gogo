mod db;
mod runner;
use std::env;

fn main() {
    let key = "GOGODB_PATH";

    let db = match env::var(key) {
        Ok(value) => db::open(value.as_str()),
        Err(_) => {
            println!("Using tmp path for gogo db, please set env var in your shell rc file or export GOGODB_PATH!");
            db::open("/tmp/gogo.db")
        }
    };

    runner::run(&db)
}
