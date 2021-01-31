use rocksdb::DB;

pub fn open(path: &str) -> DB {
    DB::open_default(path).unwrap()
}
