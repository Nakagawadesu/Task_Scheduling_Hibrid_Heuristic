mod colony;
mod manager_ant;
pub mod pherohormones;
mod worker_ant;

mod utils;

use std::i32::MAX;
use std::time::{Duration, Instant};

fn main() {
    /*##### READ FILE ###### */
    let is_proto = false;

    let pherohormones_output_dir = "/home/matheus/STG/pherohormones/frames";
    let mut file_path = String::new();
    let mut graph_name = "";
    let mut resuts_path = String::new();
    let mut n_ants = 4;

    /*##### PARAMETERS ###### */
    let alfa = 0.0;
    let beta = 1.0;

    if is_proto {
        file_path = "/home/matheus/STG/protostg/".to_owned();
        resuts_path = "/home/matheus/STG/results/protostg/".to_owned();
        // graph_name = "atest2.stg";
        graph_name = "proto151.stg"
    } else {
        let number = 2500;
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

    let base_chance = 1.0 / utils.n_tasks as f64;

    /*##### CALL AND MEASURE ###### */

    let start_time = Instant::now();

    let colony = &mut colony::Colony::new(
        &utils,
        n_ants,
        0.01,
        0.0025,
        pherohormones_output_dir,
        &resuts_path,
        graph_name,
    );
    let becnhmark: i32 = MAX;
    let best = colony.ACO(100, alfa, beta, base_chance);
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
    println!("Time to complete the work: ");
    println!("{}", best.to_string() + " cycles");
}
