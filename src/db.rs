use rocksdb::{DB, IteratorMode};

pub fn open(path: &str) -> DB {
    DB::open_default(path).unwrap()
}

pub fn list(db: &DB) {
    let iter = db.iterator(IteratorMode::Start);

    for (key, value) in iter {
        let human_key = String::from_utf8(key.to_vec()).unwrap();
        let human_value = String::from_utf8(value.to_vec()).unwrap();
        println!("name: {:?} \t url: {:?}", human_key, human_value)
    }

}
