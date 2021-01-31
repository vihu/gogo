use rocksdb::{DB, IteratorMode};
use colored::*;
use prettytable::{Table, row, cell};

pub fn open(path: &str) -> DB {
    DB::open_default(path).unwrap()
}

pub fn list(db: &DB) {
    let iter = db.iterator(IteratorMode::Start);

    let mut table = Table::new();

    table.add_row(row!["Mnemonic".yellow().bold(), "URL".yellow().bold()]);

    for (key, value) in iter {
        let human_key = String::from_utf8(key.to_vec()).unwrap();
        let human_value = String::from_utf8(value.to_vec()).unwrap();
        table.add_row(row![human_key.cyan(), human_value.green()]);
    }

    table.printstd();

}

pub fn remove(db: &DB, key: &str) {
    match db.get(key) {
        Ok(None) => println!("{}", "key does not exist".yellow()),
        Ok(_) => {
            match db.delete(key) {
                Ok(()) => {
                    println!("{} removed", key.green());
                    println!("{}", "Updated list".purple().bold());
                    list(db)
                }
                Err(_) => println!("{}", "Unable to remove key".red().bold())
            }
        },
        Err(_) => println!("{}", "Unable to remove key".red().bold())
    }

}
