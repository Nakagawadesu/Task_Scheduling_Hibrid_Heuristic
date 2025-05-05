mod colony;
mod manager_ant;
pub mod pherohormones;
mod worker_ant;

mod utils;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fmt;
use std::io::Write;
use std::time::{Duration, Instant};
use utils::Utils;
#[derive(Debug, Clone)]
struct ColonyResult {
    thread_id: i32,
    deposit_rate: f64,
    evaporation_rate: f64,
    base_chance: f64,
    max_weight: f64,
    alfa: f64,
    beta: f64,
    best_cycle: i32,
    output_dir: String,
    ants: Vec<worker_ant::WorkerAnt>,
}
impl ColonyResult {
    pub fn save_all(results: &[ColonyResult], output_dir: &str) {
        // Ensure the output directory exists
        std::fs::create_dir_all(output_dir).unwrap();

        for result in results {
            let filename = format!("{}/thread_{}/result.txt", output_dir, result.thread_id);
            let mut file = std::fs::File::create(&filename).unwrap();
            writeln!(file, "{}", result).unwrap();
        }
    }
}
impl fmt::Display for ColonyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Thread ID: {}\nDeposit Rate: {:.4}\nEvaporation Rate: {:.4}\nBase Chance: {:.4}\nMax Weight: {:.4}\nAlfa: {:.4}\nBeta: {:.4}\nBest Cycle Count: {}\nOutput Directory: {}",
            self.thread_id,
            self.deposit_rate,
            self.evaporation_rate,
            self.base_chance,
            self.max_weight,
            self.alfa,
            self.beta,
            self.best_cycle,
            self.output_dir
        )
    }
}

fn main() {
    /*##### READ FILE ###### */
    let is_proto = false;

    let mut pherohormones_output_dir = String::new();
    let mut file_path = String::new();
    let mut graph_name = String::new();
    let mut resuts_path = String::new();

    /*##### PARAMETERS ###### */
    let mut n_ants = 2;
    let epochs: i32 = 10000;
    let benchmark: i32 = 8244;

    let n_threads = 12;
    let deposit_base = 0.01;

    if is_proto {
        let number = 100;
        file_path = "/home/matheus/STG/protostg/".to_owned();
        resuts_path = format!("/home/matheus/STG/results/protostg/{}/", number).to_string();
        // graph_name = "atest2.stg";
        graph_name = format!("proto{}.stg", number).to_string();

        pherohormones_output_dir =
            format!("/home/matheus/STG/results/protostg/{}/", number).to_string();
    } else {
        let number = 3000;
        file_path = format!("/home/matheus/STG/{}/", number);
        resuts_path = format!("/home/matheus/STG/results/{}/", number).to_string();
        // graph_name = "atest2.stg";
        graph_name = "rand0000.stg".to_owned();
        pherohormones_output_dir = format!("/home/matheus/STG/results/{}/", number).to_string();
    }
    let mut utils = utils::Utils::new();

    /*##### INIT ###### */
    if is_proto {
        utils.initialize_graph_prototype(&file_path, &graph_name, &mut n_ants);
    } else {
        utils.initialize_graph(&file_path, &graph_name);
    }

    utils.init_arrays();

    // utils.print_graph();
    // utils.print_vecs();
    //base chance must be a vector of chances
    utils.init_parameters_vec(n_threads, deposit_base);
    // let parameters = vec![
    //     (0.01, 0.005),    // Original parameters 1 to 2
    //     (0.01, 0.00125),  // New combination  1 to 8
    //     (0.01, 0.000625), // New combination 1 to 16
    // ];

    /*##### CALL AND MEASURE ###### */

    let start_time = Instant::now();

    let colonies: Vec<_> = utils
        .thread_pherohormones
        .clone()
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
                &pherohormones_output_dir,
                &output_dir,
                &graph_name,
                i as i32,
                1.0,
                0.0,
                1.0,
                0.0,
            )
        })
        .collect();

    let results: Vec<ColonyResult> = colonies
        .into_par_iter()
        .map(|mut colony| {
            let (best_cycle, best_work_history) = colony.ACO(epochs, benchmark);
            ColonyResult {
                deposit_rate: colony.deposit_rate,
                evaporation_rate: colony.evaporation_rate,
                best_cycle,
                output_dir: colony.file_path.clone(),
                thread_id: colony.thread_id,
                base_chance: colony.base_chance,
                max_weight: colony.max_weight,
                alfa: colony.alfa,
                beta: colony.beta,
                ants: best_work_history,
            }
        })
        .collect();

    /*##### FIND BEST RESULT ###### */
    let best_result = results.iter().min_by_key(|r| r.best_cycle).unwrap();

    //Save the results
    ColonyResult::save_all(&results, &resuts_path);
    println!("\n=== BEST COLONY ===");
    println!("Thread ID: {}", best_result.thread_id);
    println!("Deposit Rate: {:.4}", best_result.deposit_rate);
    println!("Evaporation Rate: {:.4}", best_result.evaporation_rate);
    println!("Best Cycle Count: {}", best_result.best_cycle);
    println!("Output Directory: {}", best_result.output_dir);
    //end Time and print
    let end_time = Instant::now();

    // gantt_chart
    // Utils::print_gantt_chart(&best_result.ants);

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
