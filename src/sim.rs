use crate::config;

use config::*;

pub struct Simulation {
    models: Vec<Model>
}


impl Simulation {

    pub fn init(config: Config) -> Self {
        Simulation{ models: config.models }
    }

    /// runs the simulation for one cycle
    // TODO: This needs to run in a separate thread
    // TODO: thread calls into wasm graphics backend when necessary
    // this will probably require a simulated MMU layer in design
    pub fn run(&self) {

        for m in &self.models {
            m.eval();
        }

    }

    pub fn get(&self, var: &str) -> u8 {
        STATE.lock().unwrap().get(var).unwrap().clone()
    }

}
