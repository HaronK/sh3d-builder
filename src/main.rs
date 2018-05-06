extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate failure_derive;

use clap::{App, Arg};
use failure::ResultExt;
use std::fs::File;
use std::io::{self, Read};

mod builder;
mod input_desc;
mod output_desc;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_home_config(file_match: Option<&str>) -> Result<String, failure::Error> {
    let mut home_config = String::new();

    match file_match {
        Some(filename) => File::open(filename)
            .context(format!("Cannot open file {}", filename))?
            .read_to_string(&mut home_config)
            .context(format!("Cannot read file {}", filename))?,
        None => io::stdin().read_to_string(&mut home_config)?,
    };

    Ok(home_config)
}

fn build_home(home_config: String) -> Result<String, failure::Error> {
    let home: input_desc::Home =
        serde_json::from_str(&home_config).context("Cannot parse JSON data")?;
    Ok(serde_json::to_string(&builder::build(home)?)?)
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
            Ok(home_config) => match build_home(home_config) {
                Ok(home) => println!("OK {}", home),
                Err(err) => eprintln!("ERR {}", err),
            },
            Err(err) => eprintln!("ERR {}", err),
        };
    }
}
