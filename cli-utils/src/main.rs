extern crate cli_utils;
#[macro_use]
extern crate clap;

use cli_utils::echo::echo;
use cli_utils::wc::wc;
use cli_utils::cat::cat;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("echo") => echo(&matches),
        Some("wc")   => wc(&matches),
        Some("cat")  => cat(&matches),
        None | _     => panic!("Unexpected subcommand"),
    }
}
