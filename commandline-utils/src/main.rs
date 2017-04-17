extern crate commandline_utils;
#[macro_use]
extern crate clap;

use commandline_utils::echo::echo;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("echo") => echo(&matches),
        Some("wc")   => println!("wc"),
        None | _     => panic!("Unexpected subcommand"),
    }
}
