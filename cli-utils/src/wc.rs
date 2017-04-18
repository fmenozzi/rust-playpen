use clap::ArgMatches;

use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn wc(matches: &ArgMatches) {
    let wc_matches = matches.subcommand_matches("wc").unwrap();

    let filename = wc_matches.value_of("file").unwrap();

    match File::open(&Path::new(filename)) {
        Ok(file) => {
            let (mut num_lines, mut num_words, mut num_chars) = (0, 0, 0);
            for line in BufReader::new(&file).lines() {
                let line = line.unwrap();
                num_lines += 1;
                num_words += line.split_whitespace().count();
                num_chars += line.chars().count() + 1;
            }

            let count_lines = wc_matches.is_present("lines") as i32;
            let count_words = wc_matches.is_present("words") as i32;
            let count_chars = wc_matches.is_present("chars") as i32;

            let lwc_combo = format!("{}{}{}", count_lines, count_words, count_chars);
            match lwc_combo.as_ref() {
                "001" => println!("{} {}", num_chars, filename),
                "010" => println!("{} {}", num_words, filename),
                "100" => println!("{} {}", num_lines, filename),
                "011" => println!("{} {} {}", num_words, num_chars, filename),
                "101" => println!("{} {} {}", num_lines, num_chars, filename),
                "110" => println!("{} {} {}", num_lines, num_words, filename),

                "000" | "111" =>
                    println!("{} {} {} {}", num_lines, num_words, num_chars, filename),

                _    => (),
            }
        },
        Err(err) => {
            println!("wc: {}: {}", filename, err);
        }
    }
}
