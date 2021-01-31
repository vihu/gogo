mod db;
mod runner;

fn main() {
    dotenv::from_filename(".env").ok();

    let key = "GOGODB";

    let db = match dotenv::var(key) {
        Ok(value) => db::open(value.as_str()),
        Err(_) => db::open("/tmp/gogo")
    };

    runner::run(db)
}

