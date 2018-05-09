extern crate clap;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use failure::ResultExt;
use std::fs::File;
use std::io::{self, Read};

mod builder;
mod error;
mod input_desc;
mod output_desc;

use error::Result;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_home_config(file_match: Option<&str>) -> Result<String> {
    let mut home_config = String::new();

    match file_match {
        Some(filename) => File::open(filename)
            .context(format!("Cannot open file {}", filename))?
            .read_to_string(&mut home_config)
            .context(format!("Cannot read file {}", filename))?,
        None => io::stdin()
            .read_to_string(&mut home_config)
            .context("Cannot read from the standard input")?,
    };

    Ok(home_config)
}

fn build_home(home_config: &str, format: bool) -> Result<String> {
    let home: input_desc::Home =
        serde_json::from_str(&home_config).context("Cannot parse JSON data")?;

    let new_home = builder::build(&home)?;
    let result = if format {
        serde_json::to_string_pretty(&new_home)?
    } else {
        serde_json::to_string(&new_home)?
    };

    Ok(result)
}

fn run(show_version: bool, file_match: Option<&str>, format: bool) -> Result<String> {
    if show_version {
        Ok(VERSION.to_owned())
    } else {
        let home_config = get_home_config(file_match)?;
        build_home(&home_config, format)
    }
}

fn main() {
    let matches = App::new("Sweet Home 3D model builder")
        .version(VERSION)
        .author("Khryptul Oleg <okreptul@yahoo.com>")
        .about("Generating data for building Sweet Home 3D rooms from the human readable description.")
        .arg(
            Arg::with_name("version-number")
                .help(&format!("Prints version number: {}", VERSION))
                .short("v"),
        )
        .arg(
            Arg::with_name("format")
                .help("Format the output")
                .short("f"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input file to use")
                .index(1),
        )
        .get_matches();

    let result = run(
        matches.is_present("version-number"),
        matches.value_of("FILE"),
        matches.is_present("format"),
    );

    match result {
        Ok(home) => println!("OK {}", home),
        Err(err) => err.causes().for_each(|cause| eprintln!("ERR {}", cause)),
    }
}
