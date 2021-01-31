use clap::{App, Arg, ArgMatches};
use sled;

pub fn run(db: sled::Db) {
    let matches = matches();
    match_subcommand(db, matches)
}

fn match_subcommand(db: sled::Db, matches: ArgMatches) {
    match matches.subcommand() {
        Some(("open", open_matches)) => handle_open(db, open_matches),
        Some(("add", add_matches)) => handle_add(db, add_matches),
        None => println!("No command was used"),
        _ => unreachable!(),
    }
}

fn matches() -> ArgMatches {
    App::new("gogo")
        .about("A mnemonic url opener")
        .version("1.0")
        .subcommand(
            App::new("open")
                .about("opens mnemonic url")
                .arg(Arg::new("open").about("The url to open").takes_value(true).required(true)),
        )
        .subcommand(
            App::new("add")
                .about("add url")
                .arg(Arg::new("name").about("url name").takes_value(true).required(true))
                .arg(Arg::new("val").about("url value").takes_value(true).required(true))
        )
        .get_matches()
}

fn open_help() {
    println!("No match found, please use add command first!");
    println!("gogo add name actual_url")
}

fn insert_help(name: &str, value: &str) {
    println!("Inserting {:?} for url: {:?}", name, value)
}

fn upsert_help(prev: sled::IVec, new: &str, value: &str) {
    println!("Changing {:?} to {:?} for url: {:?}", prev, new, value)
}

fn handle_open(db: sled::Db, open_matches: &ArgMatches) {
    println!("Opening {}", open_matches.value_of("open").unwrap());
    let open_val = open_matches.value_of("open").unwrap();

    match db.get(&open_val) {
        Ok(Some(url)) => println!("{:?} -> {:?}", open_val, url),
        Ok(None) => open_help(),
        Err(_) => open_help(),
    }
}

fn handle_add(db: sled::Db, add_matches: &ArgMatches) {
    println!("name {}", add_matches.value_of("name").unwrap());
    println!("val {}", add_matches.value_of("val").unwrap());

    let name = add_matches.value_of("name").unwrap();
    let value = add_matches.value_of("val").unwrap();

    match db.insert(name, value) {
        Ok(None) => insert_help(name, value),
        Ok(Some(previous)) => upsert_help(previous, name, value),
        Err(_) => insert_help(name, value)
    }
}
