use rand::Rng;

use crate::manager_ant::{self, ManagerAnt};
use crate::pherohormones::{self, Pherohormones};
use crate::utils::Utils;
use crate::worker_ant::WorkerAnt;
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
    pub base_chance: f64,
    pub alfa: f64,
    pub beta: f64,
    pub max_weight: f64,
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
        base_chance: f64,
        alfa: f64,
        beta: f64,
        max_weight: f64,
    ) -> Colony {
        let pherohormones = Arc::new(Mutex::new(Pherohormones::new(
            utils.n_tasks,
            &format!(
                "{}/{}/thread_{}/pherohormones",
                pherohormones_output_dir, graph_name, thread_id
            ),
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
            base_chance,
            alfa,
            beta,
            max_weight,
        }
    }

    pub fn ACO(&mut self, epochs: i32, benchmark: i32) -> (i32, Vec<WorkerAnt>) {
        let file_name = format!("thread_{}_{}.csv", self.thread_id, self.graph_name);

        Utils::delete_file(&self.file_path, &file_name);
        let mut best_cycle = i32::MAX;
        let mut best_work_history: Vec<WorkerAnt> = vec![];
        //use to get the true base chance after the first cilce where the magic number comes in
        let mut max_weight = 0.0;

        // Initialize pheromones once with lock
        self.pherohormones.lock().unwrap().initialize();

        for epoch in 0..epochs {
            if epoch == 2 {
                //after the very firtst epoch the value of the max_wight will start to increadse in n* log(n) rate , so to make thnings fair
                //  and favor the ants exploration the  base chance will be proportional to this number

                let mut rng = rand::rng();
                let min_value = max_weight / 100.0;
                let max_value = 2.0 * max_weight;
                self.base_chance = rng.random_range(min_value..max_value);
                println!(
                    "[Thread {}] Base chance updated to: {}",
                    self.thread_id, self.base_chance
                );
            }
            let mut rng = rand::rng();
            //let alfa = rng.random_range(0.0..2.0);
            self.alfa = 0.0;
            // self.beta = rng.random_range(0.1..3.0);
            self.beta = 1.0;
            let mut manager = ManagerAnt::new(
                &self.utils,
                self.n_ants,
                Arc::clone(&self.pherohormones),
                self.evaporation_rate,
                self.deposit_rate,
                self.utils.n_tasks,
                self.alfa,
                self.beta,
                self.base_chance,
            );

            let cycles = manager.work(epoch);
            // Check if the cycle is valid o update the main pherohormones (deposit and evaporate)
            // Only update main pheromones if solution improves or stay the same
            if cycles <= best_cycle {
                // Get lock on main pheromones
                let mut main_ph = self.pherohormones.lock().unwrap();

                // Replace main pheromones with successful local version
                *main_ph = manager.local_pherohormones.clone();

                // Apply evaporation AFTER merging
                main_ph.evaporate_pherohormones(self.evaporation_rate);

                // Save state with lock
                main_ph.save_gephi(epoch).expect("Failed to save frame");

                // Print and save results
                // println!(
                //     "[Thread {}] Epoch {}: Cycles: {}, Max weight: {}",
                //     self.thread_id, epoch, cycles, manager.max_weight
                // );

                // Update best results
                best_cycle = cycles;
                best_work_history = manager.ants.clone();
                // Save pheromones to CSV
                let _ = Utils::append_to_csv(
                    epoch,
                    manager.max_weight,
                    cycles,
                    &self.file_path,
                    &file_name,
                );

                //  write the respective graphs for hte best
                main_ph.to_gexf(cycles);
                let _ = main_ph.save_gephi(cycles);
                self.max_weight = manager.max_weight;

                // Save the pherohormones state to view with gephi
                main_ph.save_gephi(epoch).expect("Failed to save frame");

                // Some debuggin
                println!(
                    "New best found [Thread {}] Epoch {}: Cycles: {}, Max weight: {}",
                    self.thread_id, epoch, cycles, manager.max_weight
                );
            }
            // Initialize max weight after first epoch
            if epoch == 1 {
                max_weight = manager.max_weight;
                println!(" [Thread {}] max Weight {}", self.thread_id, max_weight);
            }

            // Periodic logging
            if epoch % 100 == 0 {
                println!(
                    "[Thread {}] Progress - Epoch {}/{}: Best = {}, Current = {}",
                    self.thread_id, epoch, epochs, best_cycle, cycles
                );
            }
            // Early stopping condition
            if cycles <= benchmark {
                println!(
                    "[Thread {}] Benchmark achieved at epoch {}! Cycles: {} (Target: {})",
                    self.thread_id, epoch, cycles, benchmark
                );
                break;
            }
        }

        (best_cycle, best_work_history)
    }
}
