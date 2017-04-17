use clap::ArgMatches;

pub fn echo(matches: &ArgMatches) {
    let echo_matches = matches.subcommand_matches("echo").unwrap();

    let s = echo_matches.value_of("string").unwrap();

    if echo_matches.is_present("no_newline") {
        print!("{}", s);
    } else {
        println!("{}", s);
    }
}
