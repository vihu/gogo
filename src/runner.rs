use crate::{db, BROWSER_KEY};
use clap::{App, Arg, ArgMatches};
use colored::*;
use rocksdb::DB;
use std::process::Command;
use url::Url;
use anyhow::{anyhow, Result};

pub fn run(db: &DB) -> Result<()> {
    let matches = matches();
    match handle_default(db, &matches) {
        Ok(()) => Ok(()),
        Err(_e) => {
            match match_subcommand(db, matches) {
                Ok(()) => Ok(()),
                Err(_) => {
                    default_help();
                    Ok(())
                }
            }
        }
    }
}

fn match_subcommand(db: &DB, matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("open", open_matches)) => handle_open(db, open_matches),
        Some(("add", add_matches)) => handle_add(db, add_matches),
        Some(("search", search_matches)) => handle_search(db, search_matches),
        Some(("set_browser", set_matches)) => handle_set(db, set_matches),
        Some(("get_browser", _)) => handle_get(db),
        Some(("list", _)) => handle_list(db),
        Some(("rm", rm_matches)) => handle_rm(db, rm_matches),
        None => {
            default_help();
            Ok(())
        }
        _ => Err(anyhow!("Nothing to do"))
    }
}

fn matches() -> ArgMatches {
    App::new("gogo")
        .about("A mnemonic url opener")
        .version("1.0")
        .arg(
            Arg::new("mnemonic")
            .about("The mnemonic to open")
            .takes_value(true)
            .required(false),
        )
        .subcommand(
            App::new("open").about("Open url using mnemonic").arg(
                Arg::new("open")
                    .about("The url to open")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(
            App::new("set_browser")
                .about("Allow setting preferred browser")
                .arg(
                    Arg::new("browser")
                        .about("The browser to set")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("rm").about("Remove mnemonic").arg(
                Arg::new("rm")
                    .about("The mnemonic to remove")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(App::new("list").about("List mnemonic url mapping"))
        .subcommand(App::new("get_browser").about("Get currently configured browser"))
        .subcommand(
            App::new("add")
                .about("Add url mnemonic mapping")
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
        .subcommand(
            App::new("search")
                .about("Construct /search?q= query for known mnemonic")
                .arg(
                    Arg::new("mnemonic")
                        .about("known mnemonic")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("query")
                        .about("query to search")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches()
}

fn handle_default(db: &DB, matches: &ArgMatches) -> Result<()> {
    match matches.value_of("mnemonic") {
        Some(mnemonic) => {
            open(db, mnemonic);
            Ok(())
        }
        None => {
            Err(anyhow!("Missing default"))
        }
    }
}

fn handle_open(db: &DB, open_matches: &ArgMatches) -> Result<()> {
    let open_val = open_matches.value_of("open").unwrap();
    open(db, open_val);
    Ok(())
}

fn open(db: &DB, open_val: &str) {
    match db::get_url_from_mnemonic(db, open_val) {
        Some(actual_url) => match db::get_browser(db) {
            Some(actual_browser) => {
                println!(
                    "{} maps to {}, opening {}...",
                    open_val.green(),
                    actual_url.green(),
                    actual_browser.green()
                );
                open_browser(actual_browser, actual_url)
            }
            None => open_help(),
        },
        None => open_help(),
    }
}

fn handle_add(db: &DB, add_matches: &ArgMatches) -> Result<()> {
    let name = add_matches.value_of("name").unwrap();
    let value = add_matches.value_of("val").unwrap();

    match Url::parse(value) {
        Ok(_url) => {
            db::insert(db, name, value);
            Ok(())
        }
        Err(e) => {
            println!("{} is not a valid url", value.red().bold());
            Err(anyhow!(e.to_string()))
        }
    }
}

fn handle_list(database: &DB) -> Result<()> {
    db::list_mnemonics(database);
    Ok(())
}

fn handle_rm(db: &DB, rm_matches: &ArgMatches) -> Result<()> {
    let rm_val = rm_matches.value_of("rm").unwrap();
    db::remove(db, rm_val);
    Ok(())
}

fn handle_set(db: &DB, set_matches: &ArgMatches) -> Result<()> {
    let set_val = set_matches.value_of("browser").unwrap();
    db::insert(db, BROWSER_KEY, set_val);
    Ok(())
}

fn handle_get(database: &DB) -> Result<()> {
    match db::get_browser(database) {
        Some(actual_browser) => {
            println!("{}{}", "browser: ".green(), actual_browser.green());
            Ok(())
        }
        None => Ok(())
    }
}

fn handle_search(db: &DB, search_matches: &ArgMatches) -> Result<()> {
    let mnemonic = search_matches.value_of("mnemonic").unwrap();
    let query = search_matches.value_of("query").unwrap();

    match db::get_url_from_mnemonic(db, &mnemonic) {
        Some(actual_url) => match db::get_browser(db) {
            Some(actual_browser) => {
                let search_url = actual_url.clone() + "/search?q=" + query;

                println!(
                    "searching {} which maps to {} for {}...",
                    mnemonic.green(),
                    actual_url.green(),
                    query.green()
                );

                open_browser(actual_browser, search_url);
                Ok(())
            }
            None => {
                search_help();
                Ok(())
            }
        },
        None => {
            search_help();
            Ok(())
        }
    }
}

fn open_help() {
    println!(
        "{}",
        "No match found, please use add command first!".red().bold()
    );
    println!("{}", "gogo add name actual_url".yellow().bold())
}

fn default_help() {
    println!(
        "{}",
        "Maybe try `gogo gh`".yellow().bold()
    )
}

fn search_help() {
    println!(
        "{}",
        "No match found, please use add command first!".red().bold()
    )
}

fn open_browser(browser: String, url: String) {
    Command::new(browser)
        .arg(url)
        .spawn()
        .expect("Firefox blew up");
}

