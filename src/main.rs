mod Colony;
mod Utils;

use std::time::{Duration, Instant};

fn main() {
    /*##### READ FILE ###### */
    let is_proto = true;

    let mut file_path = "";
    let mut graph_name = "";
    let mut n_ants = 4;

    if is_proto {
        file_path = "/home/matheus/STG/protostg/";
        graph_name = "proto001.stg";
    } else {
        file_path = "/home/matheus/STG/500/";
        graph_name = "rand0147.stg";
    }
    let mut graph = Utils::Utils::new();

    /*##### INIT ###### */
    if is_proto {
        graph.initialize_graph_prototype(file_path, graph_name, &mut n_ants);
    } else {
        graph.initialize_graph(file_path, graph_name);
    }

    graph.init_arrays();

    let mut colony = Colony::Colony::new(&graph, n_ants);

    /*##### CALL AND MEASURE ###### */

    let start_time = Instant::now();

    colony.work();

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
    println!("{}", colony.current_c().to_string() + " cycles");
}
