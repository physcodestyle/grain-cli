extern crate clap;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(ref matches) = matches.subcommand_matches("run") {
        // "$ grain run" was run
        if matches.is_present("config") {
            // "$ grain test -c" was run
            println!("Running with config...");
        } else {
            println!("Running normally...");
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ grain test" was run
        if matches.is_present("debug") {
            // "$ grain test -d" was run
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
}
