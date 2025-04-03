mod colony;
mod manager_ant;
pub mod pherohormones;
mod worker_ant;

mod utils;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fmt;
use std::i32::MAX;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct ColonyResult {
    thread_id: i32,
    deposit_rate: f64,
    evaporation_rate: f64,
    best_cycle: i32,
    output_dir: String,
}

impl fmt::Display for ColonyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Deposit: {:.4}, Evap: {:.4}, Cycles: {}",
            self.deposit_rate, self.evaporation_rate, self.best_cycle
        )
    }
}

fn main() {
    /*##### READ FILE ###### */
    let is_proto = false;

    let pherohormones_output_dir = "/home/matheus/STG/pherohormones/frames";
    let mut file_path = String::new();
    let mut graph_name = "";
    let mut resuts_path = String::new();
    let mut n_ants = 4;
    let epochs: i32 = 500;

    /*##### PARAMETERS ###### */
    let alfa = 0.0;
    let beta = 1.0;

    if is_proto {
        file_path = "/home/matheus/STG/protostg/".to_owned();
        resuts_path = "/home/matheus/STG/results/protostg/".to_owned();
        // graph_name = "atest2.stg";
        graph_name = "proto151.stg"
    } else {
        let number = 300;
        file_path = format!("/home/matheus/STG/{}/", number);
        resuts_path = format!("/home/matheus/STG/results/{}/", number).to_string();
        // graph_name = "atest2.stg";
        graph_name = "rand0001.stg";
    }
    let mut utils = utils::Utils::new();

    /*##### INIT ###### */
    if is_proto {
        utils.initialize_graph_prototype(&file_path, graph_name, &mut n_ants);
    } else {
        utils.initialize_graph(&file_path, graph_name);
    }

    utils.init_arrays();

    utils.print_graph();
    utils.print_vecs();

    let base_chance = //1.0 / utils.n_tasks as f64;
    0.045;

    /*##### CALL AND MEASURE ###### */

    let start_time = Instant::now();
    let parameters = vec![
        (0.01, 0.005),    // Original parameters 1 to 2
        (0.01, 0.00125),  // New combination  1 to 8
        (0.01, 0.000625), // New combination 1 to 16
    ];

    let colonies: Vec<_> = parameters
        .into_par_iter()
        .with_min_len(1) // Force no work stealing
        .with_max_len(1) // Force 1 task per thread
        .enumerate()
        .map(|(i, (dr, er))| {
            let output_dir = format!("{}/thread_{}", resuts_path, i);
            colony::Colony::new(
                &utils,
                n_ants,
                dr,
                er,
                pherohormones_output_dir,
                &output_dir,
                graph_name,
                i as i32,
            )
        })
        .collect();

    let results: Vec<ColonyResult> = colonies
        .into_par_iter()
        .map(|mut colony| {
            let best_cycle = colony.ACO(epochs, alfa, beta, base_chance);
            ColonyResult {
                deposit_rate: colony.deposit_rate,
                evaporation_rate: colony.evaporation_rate,
                best_cycle,
                output_dir: colony.file_path.clone(),
                thread_id: colony.thread_id,
            }
        })
        .collect();

    /*##### FIND BEST RESULT ###### */
    let best_result = results.iter().min_by_key(|r| r.best_cycle).unwrap();

    println!("\n=== BEST COLONY ===");
    println!("Thread ID: {}", best_result.thread_id);
    println!("Deposit Rate: {:.4}", best_result.deposit_rate);
    println!("Evaporation Rate: {:.4}", best_result.evaporation_rate);
    println!("Best Cycle Count: {}", best_result.best_cycle);
    println!("Output Directory: {}", best_result.output_dir);
    //end Time and print
    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);
    let elapsed_seconds = elapsed_time.as_secs();
    let elapsed_millis = elapsed_time.as_millis();
    let elapsed_micros = elapsed_time.as_micros();

    /*##### PRINT RESULTS ###### */

    println!(
        "Real Time Spent: {}s {}ms {}us",
        elapsed_seconds, elapsed_millis, elapsed_micros
    );
}
