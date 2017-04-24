use clap::ArgMatches;

use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn cat(matches: &ArgMatches) {
    let cat_matches = matches.subcommand_matches("cat").unwrap();

    if let Some(files) = cat_matches.values_of("files") {
         for filename in files {
            match File::open(&Path::new(filename)) {
                Ok(file) => {
                    for line in BufReader::new(&file).lines() {
                        println!("{}", line.unwrap());
                    }
                },
                Err(err) => {
                    println!("cat: {}: {}", filename, err);
                }
            }
         }
    }
}
