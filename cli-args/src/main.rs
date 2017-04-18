extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Test Program")
                        .version("1.0")
                        .author("Federico Menozzi <federicogmenozzi@gmail.com>")
                        .about("Test this commandline arg library")
                        .arg(Arg::with_name("config")
                             .short("c")
                             .long("config")
                             .value_name("FILE")
                             .help("Sets a custom config file")
                             .takes_value(true))
                        .arg(Arg::with_name("INPUT")
                             .help("Sets the input file to use")
                             .required(true)
                             .index(1))
                        .arg(Arg::with_name("v")
                             .short("v")
                             .multiple(true)
                             .help("Sets level of verbosity"))
                        .subcommand(SubCommand::with_name("test")
                                    .about("controls testing features")
                                    .version("1.3")
                                    .author("Someone Else <someone@else.com>")
                                    .arg(Arg::with_name("debug")
                                         .short("d")
                                         .help("print debug information verbosely")))
                        .get_matches();

    // Gets value for config (defaults to "default.conf" if unspecified)
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Config: {}", config);

    // Since INPUT is required, we can safely use "unwrap()"
    println!("Input file: {}", matches.value_of("INPUT").unwrap());

    // Vary output based on verbosity level (e.g. -v vs -vvvv)
    match matches.occurrences_of("v") {
        0 => println!("Verbosity: low"),
        1 => println!("Verbosity: medium"),
        2 => println!("Verbosity: high"),
        _ => println!("Verbosity: very high"),
    }

    // Subcommands
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
}
