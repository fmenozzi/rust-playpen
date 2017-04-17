use clap::ArgMatches;

pub fn wc(matches: &ArgMatches) {
    let wc_matches = matches.subcommand_matches("wc").unwrap();

    let filename = wc_matches.value_of("file").unwrap();

    let count_chars = wc_matches.is_present("chars");
    let count_words = wc_matches.is_present("words");
    let count_lines = wc_matches.is_present("lines");

    println!("File: {}", filename);
    println!("Count chars? {}", count_chars);
    println!("Count words? {}", count_words);
    println!("Count lines? {}", count_lines);
}
