use crate::config;

use config::Config;

pub struct Simulation {
    config: Config
}


impl Simulation {

    pub fn init(config: Config) -> Self {
        Simulation{ config }
    }

    /// runs the simulation in an infinite loop
    pub fn run() {
        // TODO: calls into wasm graphics backend when necessary
        // this will probably require a simulated MMU layer in design
    }

}
