mod Colony;
mod Utils;

use std::time::{Duration, Instant};

fn main() {
    let is_proto = true;

    let mut file_path = "";
    let mut graph_name = "";
    let mut n_ants = 2;

    if is_proto {
        file_path = "/home/matheus/STG/protostg/";
        graph_name = "atest2.stg";
    } else {
        file_path = "/home/matheus/STG/500/";
        graph_name = "rand0147.stg";
    }
    let mut graph = Utils::Utils::new();

    if is_proto {
        graph.initialize_graph_prototype(file_path, graph_name, &mut n_ants);
    } else {
        graph.initialize_graph(file_path, graph_name);
    }
    graph.init_arrays();
    print!("Graph: ");
    graph.print_graph();
    graph.print_vecs();
    let start_time = Instant::now();

    //end Time and print
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let elapsed_seconds = elapsed_time.as_secs();
    let elapsed_millis = elapsed_time.as_millis();
    let elapsed_micros = elapsed_time.as_micros();

    println!(
        "Time to read file: {}s {}ms {}us",
        elapsed_seconds, elapsed_millis, elapsed_micros
    );
}
