use crate::config;

use config::*;

pub struct Simulation {
    models: Vec<Model>
}


impl Simulation {

    pub fn init(config: Config) -> Self {
        // info!("{:#?}", config);
        let mut models = vec!();
        info!("creating simulation");
        for m in config.models {
            models.push(m.order());
        }

        // info!("{:#?}", models[0]);
        Simulation{ models: models }
    }

    /// runs the simulation for one cycle
    pub fn run(&self) {

        for m in &self.models {
            m.eval();
        }

    }

}
