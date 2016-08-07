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

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("zrsbackup")
        .version("0.0.0")
        .author("Ognyan Kulev <ognyan@ognyankulev.com>")
        .about("Rust reimplementation of zbackup 1.4.x (deduplication backup tool)")
        .arg(Arg::with_name("non-encrypted")
            .long("non-encrypted")
            .display_order(1)
            .help("Repository is non-encrypted (MANDATORY FLAG; will be implemented first)"))
        .arg(Arg::with_name("password-file")
            .long("password-file")
            .takes_value(true)
            .value_name("FILE")
            .display_order(1)
            .help("Repository is encrypted with the password in FILE (not implemented)"))
        .arg(Arg::with_name("silent")
            .long("silent")
            .display_order(2)
            .help("Don not output diagnostics (not implemented)"))
        .arg(Arg::with_name("threads")
            .long("threads")
            .takes_value(true)
            .value_name("THREADS")
            .help("Use THREADS number of threads (not implemented)"))
        .arg(Arg::with_name("cache-size")
            .long("cache-size")
            .takes_value(true)
            .value_name("SIZE")
            .help("Use SIZE MB for the cache (not implemented)"))
        .arg(Arg::with_name("exchange")
            .long("exchange")
            .takes_value(true)
            .value_name("SUBDIRECTORY")
            .possible_values(&["backups", "bundles", "index"])
            .help("Exchange SUBDIRECTORY data with other repository (not implemented)"))
        .arg(Arg::with_name("compression")
            .long("compression")
            .takes_value(true)
            .value_name("ALGORITHM")
            .possible_values(&["lzma", "lzo"])
            .help("Compression algorithm to be used (not implemented)"))
        .subcommand(SubCommand::with_name("init")
            .about("Initialize repository in REPOSITORY (will be implemented first)")
            .display_order(1)
            .arg(Arg::with_name("repository")
                .value_name("REPOSITORY")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("backup")
            .about("Backup file and name it NAME (will be implemented first)")
            .display_order(2)
            .arg(Arg::with_name("name")
                .value_name("NAME")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("restore")
            .about("Restore backup NAME (will be implemented second)")
            .display_order(3)
            .arg(Arg::with_name("name")
                .value_name("NAME")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("export")
            .about("Export from one source repository to other destination repository (not implemented)")
            .display_order(4)
            .arg(Arg::with_name("source")
                .value_name("SOURCE")
                .required(true)
                .index(1))
            .arg(Arg::with_name("destination")
                .value_name("DESTINATION")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("import")
            .about("Import from one source repository to other destination repository (not implemented)")
            .display_order(5)
            .arg(Arg::with_name("source")
                .value_name("SOURCE")
                .required(true)
                .index(1))
            .arg(Arg::with_name("destination")
                .value_name("DESTINATION")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("gc")
            .about("Garbage collection (not implemented)")
            .display_order(6)
            .arg(Arg::with_name("type")
                .value_name("TYPE")
                .required(true)
                .possible_values(&["fast", "deep"])
                .index(1))
            .arg(Arg::with_name("repository")
                .value_name("REPOSITORY")
                .required(true)
                .index(2)))
        .get_matches();


    match matches.occurrences_of("non-encrypted") + matches.occurrences_of("password-file") {
        1 => if matches.occurrences_of("non-encrypted") == 1 {
            println!("Non-encrypted repository")
        } else {
            match matches.value_of("password-file") {
                Some(pass_file) => println!("Encrypted repository with password from file `{}` (not supported yet)", pass_file),
                None => panic!("No password file given")
            }
        },
        _ => panic!("Exactly one of --non-encrypted or --password-file FILE must be present"),
    }

    // TODO check all other parameters

    // TODO do work
}

