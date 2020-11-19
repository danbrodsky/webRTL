#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate web_sys;

/// WASM

extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use std::panic;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_use]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn test_simulator() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    panic::set_hook(Box::new(console_error_panic_hook::hook));
    run();
}


/// LOGGING

struct WebLogger;

use log::{Record, Level, Metadata, Log, LevelFilter};
impl Log for WebLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            console_log!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: WebLogger = WebLogger;


/// ENTRY

pub mod config;
pub mod sim;

fn run() {

    let config_data =
r#".model counter
.inputs clock
.outputs out[0] out[1] out[2] out[3] out[4] out[5] out[6] out[7] out[8]
.names out[1] out[0] $0\out[8:0][1]
01 1
10 1
.names out[2] out[1] out[0] $0\out[8:0][2]
011 1
100 1
101 1
110 1
.names out[3] out[2] out[1] out[0] $0\out[8:0][3]
0111 1
1000 1
1001 1
1010 1
1011 1
1100 1
1101 1
1110 1
.names out[4] $abc$221$n24 $0\out[8:0][4]
01 1
10 1
.names out[3] out[2] out[1] out[0] $abc$221$n24
1111 1
.names out[5] out[4] $abc$221$n24 $0\out[8:0][5]
011 1
100 1
101 1
110 1
.names out[6] out[5] out[4] $abc$221$n24 $0\out[8:0][6]
0111 1
1000 1
1001 1
1010 1
1011 1
1100 1
1101 1
1110 1
.names out[7] $abc$221$n28 $0\out[8:0][7]
01 1
10 1
.names out[6] out[5] out[4] $abc$221$n24 $abc$221$n28
1111 1
.names out[8] out[7] $abc$221$n28 $0\out[8:0][8]
011 1
100 1
101 1
110 1
.names out[0] $0\out[8:0][0]
0 1
.latch $0\out[8:0][0] out[0] re clock 2
.latch $0\out[8:0][1] out[1] re clock 2
.latch $0\out[8:0][2] out[2] re clock 2
.latch $0\out[8:0][3] out[3] re clock 2
.latch $0\out[8:0][4] out[4] re clock 2
.latch $0\out[8:0][5] out[5] re clock 2
.latch $0\out[8:0][6] out[6] re clock 2
.latch $0\out[8:0][7] out[7] re clock 2
.latch $0\out[8:0][8] out[8] re clock 2
"#;

    let config = config::Config::new(&config_data);

    let sim = sim::Simulation::init(config);

    sim.run();

    // TODO: Entry into program
}
