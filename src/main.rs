// Copyright (c) 2016 Ognyan Kulev <ognyan@ognyankulev.com>
//
// This file is part of zrsbackup <https://github.com/okulev/zrsbackup>.
//
// zrsbackup is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// zrsbackup is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with zrsbackup.  If not, see <http://www.gnu.org/licenses/>.

extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("zrsbackup")
        .version("0.0.0")
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

