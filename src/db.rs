use sled;

pub fn open(val: &str) -> sled::Db {
    sled::open(val).expect("open")
}
