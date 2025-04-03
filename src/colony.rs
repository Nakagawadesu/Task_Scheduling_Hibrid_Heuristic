use crate::manager_ant::{self, ManagerAnt};
use crate::pherohormones::Pherohormones;
use crate::utils::Utils;
use std::sync::{Arc, Mutex};

pub struct Colony {
    pub utils: Utils,
    pub pherohormones: Arc<Mutex<Pherohormones>>, // Changed to thread-safe type
    pub deposit_rate: f64,
    pub evaporation_rate: f64,
    pub n_ants: i32,
    pub file_path: String,
    pub graph_name: String,
    pub thread_id: i32,
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
        thread_id: i32,
    ) -> Colony {
        let pherohormones = Arc::new(Mutex::new(Pherohormones::new(
            utils.n_tasks,
            &format!("{}/thread_{}", pherohormones_output_dir, thread_id),
        )));
        Colony {
            utils: utils.clone(),
            pherohormones,
            n_ants,
            deposit_rate,
            evaporation_rate,
            file_path: file_path.to_string(),
            graph_name: graph_name.to_string(),
            thread_id,
        }
    }

    pub fn ACO(&mut self, epochs: i32, alfa: f64, beta: f64, base_chance: f64) -> i32 {
        let mut best_cycle = i32::MAX;

        // Initialize pheromones once with lock
        self.pherohormones.lock().unwrap().initialize();

        for epoch in 0..epochs {
            let mut manager = ManagerAnt::new(
                &self.utils,
                self.n_ants,
                Arc::clone(&self.pherohormones),
                self.evaporation_rate,
                self.deposit_rate,
                self.utils.n_tasks,
                alfa,
                beta,
                base_chance,
            );

            let cycles = manager.work(epoch);

            // Evaporation with lock
            self.pherohormones
                .lock()
                .unwrap()
                .evaporate_pherohormones(self.evaporation_rate);

            if cycles < best_cycle {
                best_cycle = cycles;
            }

            // Save state with lock
            self.pherohormones
                .lock()
                .unwrap()
                .save_gephi(epoch)
                .expect("Failed to save frame");

            // Print and save results
            println!(
                "[Thread {}] Epoch {}: Cycles: {}, Max weight: {}",
                self.thread_id, epoch, cycles, manager.max_weight
            );
            let mut file_name = format!("thread_{}_{}.csv", self.thread_id, self.graph_name);
            let _ = Utils::append_to_csv(
                epoch,
                manager.max_weight,
                cycles,
                &self.file_path,
                &file_name,
            );
            // TO DO  write the respective graphs for hte best
            //self.pherohormones.lock().unwrap().to_gexf(cycles
        }

        best_cycle
    }
}
