#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

#[macro_use]
pub mod util;

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
use crate::util::*;

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



    let mut i = 0;
    sim.run();
    for _ in 0..640 {
        // i += 1;
        // if i > 4 {
        //     i = 4;
        // }

      set("clk", 1);
      set("en", 1);
      // set_n_to_m("c$arg_0", 0, 8, config::to_bit_vec(i));
        // set_n_to_m("c$arg_1", 0, 8, config::to_bit_vec(i));
        sim.run();
    }

    // TODO: Entry into program
}
