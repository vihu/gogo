use crate::db;
use anyhow::{anyhow, Context, Result};
use clap::{Arg, ArgMatches, Command};
use csv::{Reader, Writer};
use rusqlite::Connection;
use std::{
    process,
    time::{SystemTime, UNIX_EPOCH},
};
use tabled::Table;
use url::Url;

pub fn run(conn: &Connection) -> Result<()> {
    let matches = matches();

    if let Some(mnemonic) = matches.get_one::<String>("mnemonic") {
        let value = db::get(conn, mnemonic).with_context(|| {
            format!(
                "No associated value for key: {:?}! Maybe try gogo add {:?} url",
                mnemonic, mnemonic
            )
        })?;
        println!("opening: {:?}", value);
        open_browser(conn, value)?;
        Ok(())
    } else {
        match match_subcommand(conn, matches) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

fn open_browser(conn: &Connection, url: String) -> Result<()> {
    let browser = db::get(conn, "_browser")
        .with_context(|| "No browser set! Maybe try gogo set_browser browser_executable")?;
    process::Command::new(browser).arg(url).spawn()?;
    Ok(())
}

fn handle_add(conn: &Connection, add_matches: &ArgMatches) -> Result<()> {
    if let Some(name) = add_matches.get_one::<String>("name") {
        if let Some(value) = add_matches.get_one::<String>("val") {
            Url::parse(value)?;
            db::insert(conn, name, value)?;
        }
    }
    Ok(())
}

fn handle_import_csv(conn: &Connection, csv_matches: &ArgMatches) -> Result<()> {
    if let Some(csv_path) = csv_matches.get_one::<String>("csv") {
        let mut rdr = Reader::from_path(csv_path)?;
        for result in rdr.deserialize() {
            let record: db::Mnemonic = result?;
            db::insert(conn, &record.key, &record.val)?;
        }
    }
    Ok(())
}

fn handle_export_csv(conn: &Connection) -> Result<()> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?.as_millis();
    let fname = format!("/tmp/gogo_{:?}.csv", since_the_epoch);
    let mut wtr = Writer::from_path(&fname)?;
    let mnemonics = db::list_all(conn)?;
    for mnemonic in mnemonics {
        wtr.serialize(mnemonic)?;
    }
    println!("Output written to: {:?}", fname);
    Ok(())
}

fn handle_set_browser(conn: &Connection, add_matches: &ArgMatches) -> Result<()> {
    if let Some(value) = add_matches.get_one::<String>("browser") {
        db::insert(conn, "_browser", value)?;
    }
    Ok(())
}

fn handle_get_browser(conn: &Connection) -> Result<()> {
    let value = db::get(conn, "_browser")
        .with_context(|| "No browser set! Maybe try gogo set_browser browser_executable")?;
    println!("browser: {:?}", value);
    Ok(())
}

fn handle_rm(conn: &Connection, rm_matches: &ArgMatches) -> Result<()> {
    if let Some(name) = rm_matches.get_one::<String>("rm") {
        db::remove(conn, name)?
    }
    Ok(())
}

fn handle_check(conn: &Connection, check_matches: &ArgMatches) -> Result<()> {
    if let Some(name) = check_matches.get_one::<String>("check") {
        let value = db::get(conn, name).with_context(|| {
            format!(
                "No associated value for key: {:?}! Maybe try gogo add {:?} url",
                name, name
            )
        })?;
        println!("value: {:?}", value);
    }
    Ok(())
}

fn handle_search(conn: &Connection, search_matches: &ArgMatches) -> Result<()> {
    if let Some(name) = search_matches.get_one::<String>("mnemonic") {
        let mut value = db::get(conn, name).with_context(|| {
            format!(
                "No associated value for key: {:?}! Maybe try gogo add {:?} url",
                name, name
            )
        })?;
        if let Some(item) = search_matches.get_one::<String>("query") {
            let search_term = format!("/search?q={}", item);
            value.push_str(&search_term);
            println!("opening: {:?}", value);
            open_browser(conn, value)?;
        }
    }
    Ok(())
}

fn handle_ls(conn: &Connection) -> Result<()> {
    let mnemonics = db::list_all(conn)?;
    let table = Table::new(mnemonics).to_string();
    println!("{}", table);
    Ok(())
}

fn handle_open(conn: &Connection, open_matches: &ArgMatches) -> Result<()> {
    if let Some(key) = open_matches.get_one::<String>("open") {
        let value = db::get(conn, key).with_context(|| {
            format!(
                "No associated value for key: {:?}! Maybe try gogo add {:?} url",
                key, key
            )
        })?;
        println!("opening: {:?}", value);
        open_browser(conn, value)?;
    }
    Ok(())
}

fn match_subcommand(conn: &Connection, matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("add", add_matches)) => handle_add(conn, add_matches),
        Some(("rm", rm_matches)) => handle_rm(conn, rm_matches),
        Some(("ls", _)) => handle_ls(conn),
        Some(("open", open_matches)) => handle_open(conn, open_matches),
        Some(("check", check_matches)) => handle_check(conn, check_matches),
        Some(("search", search_matches)) => handle_search(conn, search_matches),
        Some(("set_browser", set_matches)) => handle_set_browser(conn, set_matches),
        Some(("get_browser", _)) => handle_get_browser(conn),
        Some(("import", csv_matches)) => handle_import_csv(conn, csv_matches),
        Some(("export", _)) => handle_export_csv(conn),
        _ => Err(anyhow!("Unsupported argument!")),
    }
}

fn matches() -> ArgMatches {
    Command::new("gogo")
        .about("A mnemonic url opener")
        .version("1.0")
        .arg(
            Arg::new("mnemonic")
                .help("The mnemonic to open")
                .required(false),
        )
        .subcommand(
            Command::new("open")
                .about("Open url using mnemonic")
                .arg(Arg::new("open").help("The url to open").required(true)),
        )
        .subcommand(
            Command::new("set_browser")
                .about("Allow setting preferred browser")
                .arg(
                    Arg::new("browser")
                        .help("The browser to set")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove mnemonic")
                .arg(Arg::new("rm").help("The mnemonic to remove").required(true)),
        )
        .subcommand(
            Command::new("check").about("Check mnemonic").arg(
                Arg::new("check")
                    .help("The mnemonic to check")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("import")
                .about("Import CSV")
                .arg(Arg::new("csv").help("The CSV to import").required(true)),
        )
        .subcommand(Command::new("ls").about("List mnemonic url mapping"))
        .subcommand(Command::new("get_browser").about("Get currently configured browser"))
        .subcommand(Command::new("export").about("Export database to CSV"))
        .subcommand(
            Command::new("search")
                .about("Construct /search?q= query for known mnemonic")
                .arg(Arg::new("mnemonic").help("known mnemonic").required(true))
                .arg(Arg::new("query").help("query to search").required(true)),
        )
        .subcommand(
            Command::new("add")
                .about("Add url mnemonic mapping")
                .arg(Arg::new("name").help("url name").required(true))
                .arg(Arg::new("val").help("url value").required(true)),
        )
        .get_matches()
}
