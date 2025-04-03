use crate::manager_ant::{self, ManagerAnt};

use crate::pherohormones::Pherohormones;
use crate::utils::Utils;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Colony {
    utils: Utils,
    pherohormones: Rc<RefCell<Pherohormones>>,
    deposit_rate: f64,
    evaporation_rate: f64,
    n_ants: i32,
    file_path: String,
    graph_name: String,
}

impl Colony {
    pub fn new(
        utils: &Utils,
        n_ants: i32,
        deposit_rate: f64,
        evaporation_rate: f64,
        pherohormones_output_dir: &str,
        file_path: &str,
        graph_name: &str,
    ) -> Colony {
        let pherohormones = Rc::new(RefCell::new(Pherohormones::new(
            utils.n_tasks,
            pherohormones_output_dir,
        )));
        Colony {
            utils: utils.clone(),
            pherohormones,
            n_ants,
            deposit_rate,
            evaporation_rate,
            file_path: file_path.to_string(),
            graph_name: graph_name.to_string(),
        }
    }

    pub fn ACO(&mut self, epochs: i32, alfa: f64, beta: f64, base_chance: f64) -> i32 {
        let mut best_cycle = i32::MAX;
        let mut content = String::new();

        // Initialize pheromones once
        self.pherohormones.borrow_mut().initialize();

        for epoch in 0..epochs {
            let mut manager = ManagerAnt::new(
                &self.utils,
                self.n_ants,
                Rc::clone(&self.pherohormones),
                self.evaporation_rate,
                self.deposit_rate,
                self.utils.n_tasks,
                alfa,
                beta,
                base_chance,
            );

            let cycles = manager.work(epoch);

            // evaporation after each epoch
            self.pherohormones
                .borrow_mut()
                .evaporate_pherohormones(self.evaporation_rate);

            if cycles < best_cycle {
                best_cycle = cycles;
            }

            // save state
            self.pherohormones
                .borrow()
                .save_gephi(epoch)
                .expect("Failed to save frame");

            // print max weight encountered
            println!(
                "Epoch {}:cycles : {}, Max weight encountered: {}",
                epoch, cycles, manager.max_weight
            );
            let _ = Utils::append_to_csv(
                epoch,
                manager.max_weight,
                cycles,
                &self.file_path,
                &self.graph_name,
            );
        }

        content.push_str(&format!(
            "Best cycle: {}, pheromone deposit rate: {}, pheromone evaporation rate: {}\n",
            best_cycle, self.deposit_rate, self.evaporation_rate
        ));
        best_cycle
    }
}
