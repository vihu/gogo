// gogo open mygit
// gogo add mygit "github.com/vihu"

use clap::{App, Arg};

fn main() {
    let matches = App::new("gogo")
        .about("A mnemonic url opener")
        .version("1.0")
        .author("Rahul Garg")
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
        .get_matches();

    match matches.subcommand_name() {
        Some("open") => println!("'gogo open' was used"),
        Some("add") => println!("'gogo add' was used"),
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }

    match matches.subcommand() {
        Some(("open", open_matches)) => {
            println!("Opening {}", open_matches.value_of("open").unwrap());
        }
        Some(("add", add_matches)) => {
            println!("name {}", add_matches.value_of("name").unwrap());
            println!("val {}", add_matches.value_of("val").unwrap());
        }
        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
}

