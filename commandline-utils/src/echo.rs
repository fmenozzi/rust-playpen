use clap::ArgMatches;

pub fn echo(matches: &ArgMatches) {
    let echo_matches = matches.subcommand_matches("echo").unwrap();

    if let Some(vals) = echo_matches.values_of("strings") {
        for val in vals {
            print!("{} ", val);
        }
    }

    if !echo_matches.is_present("no_newline") {
        println!("");
    }
}
