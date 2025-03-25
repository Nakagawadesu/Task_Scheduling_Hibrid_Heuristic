use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error, Write};

use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use petgraph::Direction;

use rand::thread_rng;
use random_choice::random_choice;

use crate::Utils::Utils;

#[derive(Clone)]
struct WorkeraAnt {
    // the identifyer of the worker is the index of the array
    current_task: i32,
    free_at: i32,
}
impl WorkeraAnt {
    fn new() -> WorkeraAnt {
        WorkeraAnt {
            // the values for the current task are -1 which means free and  any other value higher than -1 means busy
            current_task: -1, //free
            // the value for free_at  means the cycle that the worker will be free or better iteration
            free_at: -1,
        }
    }
}
pub(crate) struct Colony {
    n_ants: i32,
    ants: Vec<WorkeraAnt>,
    //utils: Utils
    di_graph: StableDiGraph<i32, i32>,
    n_tasks: i32,
    remaining_vec: Vec<i32>,
    costs_vec: Vec<i32>,
    available_tasks: Vec<bool>,
    visibility_vec: Vec<f64>,
    current_cycle: i32,
}

impl Colony {
    pub fn new(utils: &Utils, n_ants: i32) -> Colony {
        Colony {
            n_ants: n_ants,
            ants: vec![WorkeraAnt::new(); n_ants as usize],
            // the utils struct is passed by value and we just need to clone it because threre wuill be a lot of colonies and we dont want to share the same utils struct
            //utils: utils.clone(),
            di_graph: utils.di_graph.clone(),
            n_tasks: utils.n_tasks,
            remaining_vec: utils.remaining_vec.clone(),
            costs_vec: utils.costs_vec.clone(),
            available_tasks: vec![false; utils.n_tasks as usize],
            visibility_vec: utils.visibility.clone(),
            current_cycle: 0,
        }
    }
    // this is the main fucntion for the colony to work
    // it will be called by the main function in a loop that stops when the graph is totally destroyed meaning that all tasks are completed
    pub fn work(&mut self) {
        self.init_available_tasks();

        self.check_available_tasks();
        self.print_ants();
        println!("Starting work session...");

        while self.di_graph.node_count() > 0 {
            println!("\n Current cycle: {}", self.current_cycle);
            self.print_available_tasks();
            self.check_tasks_completion();
            self.check_available_tasks();
            self.current_cycle += 1;
        }
    }

    fn check_tasks_completion(&mut self) {
        for i in 0..self.n_ants {
            println!(
                "Ant {} (>*~*)> {} |  :) -> {}",
                i, self.ants[i as usize].current_task, self.ants[i as usize].free_at
            );
            if self.ants[i as usize].free_at <= self.current_cycle {
                // Set the worker as free.
                let finished_task = self.ants[i as usize].current_task;

                // there is a task to mark as completed so:
                // Reduce the remaining counts for neighbors of the finished task.
                // Remove the task node from the graph as well as its edges.
                println!("Task {} is completed!", finished_task);
                self.reduce_and_destroy(finished_task);

                self.ants[i as usize].current_task = -1;
                self.ants[i as usize].free_at = -1;
            }
        }
    }
    // this fucntion will allow simple setup to initiate the algorithm for both nbormal and prototype graphs, since prototype may have multiple opossible starting points
    fn init_available_tasks(&mut self) {
        for i in 0..self.n_tasks {
            if self.remaining_vec[i as usize] == 0 {
                self.available_tasks[i as usize] = true;
            } else {
                self.available_tasks[i as usize] = false;
            }
        }
    }

    fn reduce_and_destroy(&mut self, task: i32) {
        if task < 0 {
            return;
        }

        let mut neighboors = Vec::new();
        // get the neighboors of the task , they are all the outgoing edges targets
        let mut neighboors_iter = self
            .di_graph
            .neighbors_directed(NodeIndex::new(task as usize), Direction::Outgoing);

        while let Some(neighboor) = neighboors_iter.next() {
            neighboors.push(neighboor.index() as i32);
        }

        //reduce the remaining vec for the neighboors
        for &nb in &neighboors {
            self.remaining_vec[nb as usize] -= 1;
            if self.remaining_vec[nb as usize] == 0 {
                println!("Task {} is now available!", nb);
                self.available_tasks[nb as usize] = true;
            }
        }
        // it is completed so now we can remove the node from the graph and its edges
        self.di_graph.remove_node(NodeIndex::new(task as usize));
        // once a node is removed all incoming and outgoing edges are removed as well
    }

    fn check_available_tasks(&mut self) {
        for i in 0..self.n_ants {
            if self.ants[i as usize].current_task == -1 {
                // choose a taskthread_rng
                self.choose_task_randomly_weighted(i);
            }
        }
    }

    // This function chooses a task randomly among available tasks,
    // using the visibility vector as the weight (plus an optional base chance if desired).
    fn choose_task_randomly_weighted(&mut self, freeAnt: i32) {
        // Collect available tasks and their corresponding visibility weights
        let mut candidate_tasks = Vec::new();
        let mut weights = Vec::new();

        for (i, &is_available) in self.available_tasks.iter().enumerate() {
            if is_available {
                candidate_tasks.push(i);
                // For now only use the visibility vector as the weight
                // TO DO - Add an optional base chance and Pherohormones
                weights.push(self.visibility_vec[i]);
            }
        }

        // Ensure there are available tasks to choose from
        if candidate_tasks.is_empty() {
            return;
        }

        // Determine the number of choices to make; in this case, we choose one task
        let number_of_choices: usize = 1;

        // Use random_choice to select tasks based on weights
        let chosen_tasks =
            random_choice().random_choice_f64(&candidate_tasks, &weights, number_of_choices);

        // Assign the chosen task to an available ant
        if let Some(&chosen_task) = chosen_tasks.first() {
            // free ant already checked passed as parameter

            self.ants[freeAnt as usize].current_task = *chosen_task as i32;
            // Set the time when the ant will be free again; adjust as necessary
            self.ants[freeAnt as usize].free_at =
                self.current_cycle + self.costs_vec[*chosen_task as usize];
            // Mark the task as no longer available
            self.available_tasks[*chosen_task] = false;

            println!("Ant {} has Started task {} ", freeAnt, chosen_task);
        }
    }

    /*########## UTILS ########## */

    pub fn current_c(&self) -> i32 {
        self.current_cycle
    }

    pub fn print_ants(&self) {
        for i in 0..self.n_ants {
            println!(
                "Ant {} is working on task {} and will be free at cycle {}",
                i, self.ants[i as usize].current_task, self.ants[i as usize].free_at
            );
        }
    }
    pub fn print_available_tasks(&self) {
        println!("Available tasks:");
        for i in 0..self.n_tasks {
            if self.available_tasks[i as usize] {
                println!(" {}", i);
            }
        }
        print!("\n");
    }
}
