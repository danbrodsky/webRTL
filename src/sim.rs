use crate::config;

use config::*;

pub struct Simulation {
    models: Vec<Model>
}


impl Simulation {

    pub fn init(config: Config) -> Self {
        let mut models = vec!();
        info!("creating simulation");
        for m in config.models {
            models.push(m.order());
        }

        info!("{:#?}", models[0]);
        Simulation{ models: models }
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

}
