extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_home_config(file_match: Option<&str>) -> io::Result<String> {
    let mut home_config = String::new();

    match file_match {
        Some(filename) => File::open(filename)?.read_to_string(&mut home_config)?,
        None => io::stdin().read_to_string(&mut home_config)?,
    };

    Ok(home_config)
}

fn main() {
    let matches = App::new("Sweet Home 3D model builder")
        .version(VERSION)
        .author("Khryptul Oleg <dark.haron@gmail.com>")
        .about("Generating data for Sweet Home 3D from the human readable description")
        .arg(
            Arg::with_name("version-number")
                .help("Get version number")
                .short("v"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input file to use")
                .index(1),
        )
        .get_matches();

    if matches.is_present("version-number") {
        println!("{}", VERSION);
    } else {
        match get_home_config(matches.value_of("FILE")) {
            Ok(home_config) => println!("OK {}", home_config),
            Err(err) => eprintln!("ERR {}", err),
        };
    }
}
