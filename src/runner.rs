use clap::{App, Arg, ArgMatches};
use rocksdb::DB;
use std::process::Command;
use url::Url;

pub fn run(db: DB) {
    let matches = matches();
    match_subcommand(db, matches)
}

fn match_subcommand(db: DB, matches: ArgMatches) {
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
            App::new("open").about("opens mnemonic url").arg(
                Arg::new("open")
                    .about("The url to open")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(
            App::new("add")
                .about("add url")
                .arg(
                    Arg::new("name")
                        .about("url name")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("val")
                        .about("url value")
                        .takes_value(true)
                        .required(true),
                ),
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

fn handle_open(db: DB, open_matches: &ArgMatches) {
    let open_val = open_matches.value_of("open").unwrap();

    match db.get(&open_val) {
        Ok(Some(url)) => {
            let actual_url = String::from_utf8(url).unwrap();
            println!(
                "{:?} maps to {:?}, opening firefox...",
                open_val, actual_url
            );
            Command::new("firefox")
                .arg(actual_url)
                .spawn()
                .expect("Firefox blew up");
        }
        Ok(None) => open_help(),
        Err(_) => open_help(),
    }
}

fn handle_add(db: DB, add_matches: &ArgMatches) {
    println!("name {}", add_matches.value_of("name").unwrap());
    println!("val {}", add_matches.value_of("val").unwrap());

    let name = add_matches.value_of("name").unwrap();
    let value = add_matches.value_of("val").unwrap();

    match Url::parse(value) {
        Ok(_url) => match db.put(name, value) {
            Ok(_) => insert_help(name, value),
            Err(_) => insert_help(name, value),
        },
        _ => println!("{:?} is not a valid url", value),
    }
}
