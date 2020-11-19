#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

pub mod config;
pub mod sim;

/// WASM

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

struct SimpleLogger;

/// LOGGING

use log::{Record, Level, Metadata, Log, LevelFilter};
impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

/// ENTRY

use clap::{crate_version, App, Arg};

fn main() {

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

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

    sim.run();

    // TODO: Entry into program
}
