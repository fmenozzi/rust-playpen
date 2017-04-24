use clap::ArgMatches;

pub fn echo(matches: &ArgMatches) {
    let echo_matches = matches.subcommand_matches("echo").unwrap();

    if let Some(strings) = echo_matches.values_of("strings") {
        for string in strings {
            print!("{} ", string);
        }
    }

    if !echo_matches.is_present("no_newline") {
        println!("");
    }
}
