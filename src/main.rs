// Copyright (c) 2016 Ognyan Kulev <ognyan@ognyankulev.com>. All rights reserved.

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("zrsbackup")
                          .version("0.0")
                          .author("Ognyan Kulev <ognyan@ognyankulev.com>")
                          .about("Rust reimplementation of zbackup 1.4.x (deduplication backup tool)")
                          .arg(Arg::with_name("non-encrypted")
                               .long("non-encrypted")
                               .help("Repository is non-encrypted"))
                          .arg(Arg::with_name("COMMAND")
                               .help("One of: init, backup")
                               .required(true)
                               .index(1))
                          .get_matches();


    match matches.occurrences_of("non-encrypted") {
      1 => println!("Will not encrypt repository"),
      _ => panic!("--non-encrypted option is mandatory"),
    }

    match matches.value_of("COMMAND") {
      Some("init") => println!("init repository"),
      Some("backup") => println!("backup repository"),
      Some(invalid_command) => println!("Invalid command: {}", invalid_command),
      None => println!("Missing command"),
    }
}

