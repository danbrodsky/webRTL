#[macro_use]
extern crate nom;
#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod sim;

use clap::{crate_version, App, Arg};

fn main() {
    let args = App::new("webrtl")
        .version(crate_version!())
        .author("Daniel Brodsky <dnbrdsky@gmail.com>")
        .arg(
            Arg::with_name("blif")
                .help("flattened blif config")
                .required(true),
        )
        .get_matches();

    let config_data = std::fs::read_to_string(args.value_of("blif").unwrap()).expect("blif file being read");
    let config = config::Config::new(&config_data);

    let sim = sim::Simulation::init(config);

    // sim.run()

    // TODO: Entry into program
    println!("Hello, world!");
}
