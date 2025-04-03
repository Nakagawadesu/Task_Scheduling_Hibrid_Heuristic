use std::cell::RefCell;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error, Write};
use std::rc::Rc;

use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use petgraph::Direction;

use rand::thread_rng;
use random_choice::random_choice;

use crate::pherohormones::{self, Pherohormones};
use crate::utils::Utils;
use crate::worker_ant::WorkerAnt;

pub(crate) struct ManagerAnt {
    n_ants: i32,
    ants: Vec<WorkerAnt>,
    //utils: Utils
    di_graph: StableDiGraph<i32, i32>,
    n_tasks: i32,
    remaining_vec: Vec<i32>,
    costs_vec: Vec<i32>,
    available_tasks: Vec<bool>,
    visibility_vec: Vec<f64>,
    visibility_sum: f64,
    current_cycle: i32,
    pherohormones: Rc<RefCell<Pherohormones>>,
    evaporation_rate: f64,
    deposit_rate: f64,
    alfa: f64,
    beta: f64,
    base_chance: f64,
    // debug variables
    pub max_weight: f64,
}

impl ManagerAnt {
    pub fn new(
        utils: &Utils,
        n_ants: i32,
        pherohormones: Rc<RefCell<Pherohormones>>,
        evaporation_rate: f64,
        deposit_rate: f64,
        n_tasks: i32,
        alfa: f64,
        beta: f64,
        base_chance: f64,
    ) -> ManagerAnt {
        ManagerAnt {
            n_ants: n_ants,
            ants: vec![WorkerAnt::new(n_tasks); n_ants as usize],
            // the utils struct is passed by value and we just need to clone it because threre wuill be a lot of colonies and we dont want to share the same utils struct
            //utils: utils.clone(),
            di_graph: utils.di_graph.clone(),
            n_tasks: utils.n_tasks,
            remaining_vec: utils.remaining_vec.clone(),
            costs_vec: utils.costs_vec.clone(),
            available_tasks: vec![false; utils.n_tasks as usize],
            visibility_vec: utils.visibility.clone(),
            visibility_sum: utils.visibility_sum,
            current_cycle: 0,
            pherohormones,
            evaporation_rate,
            deposit_rate,
            alfa,
            beta,
            base_chance,
            max_weight: 0.0,
        }
    }
    // this is the main fucntion for the colony to work
    // it will be called by the main function in a loop that stops when the graph is totally destroyed meaning that all tasks are completed
    pub fn work(&mut self, frame_counter: i32) -> i32 {
        let mut cycles_spent = 0;

        self.init_available_tasks();

        self.check_available_tasks();

        //self.print_ants();
        //println!("Starting work session...");

        while self.di_graph.node_count() > 0 {
            //println!("\n Current cycle: {}", self.current_cycle);
            //self.print_available_tasks();

            self.check_tasks_completion();
            self.check_available_tasks();
            self.current_cycle += 1;
        }

        // Save the pherohormones state
        self.pherohormones
            .borrow()
            .save_gephi(frame_counter)
            .expect("Failed to save frame");
        // It adds 1 to the last cycle , sothe real number of cycles spent is the current cycle - 1
        cycles_spent = self.current_cycle - 1;
        cycles_spent
    }

    fn check_tasks_completion(&mut self) {
        for i in 0..self.n_ants {
            //simple prutn to check the ants status
            // println!(
            //     "Ant : {}| lastTask {} (>*~*)> {} |  FreeAt -> {}",
            //     i,
            //     if self.ants[i as usize].last_task == -1 {
            //         -1
            //     } else {
            //         self.ants[i as usize].last_task + 1
            //     },
            //     if self.ants[i as usize].current_task == -1 {
            //         -1
            //     } else {
            //         self.ants[i as usize].current_task + 1
            //     },
            //     self.ants[i as usize].free_at
            // );

            if self.ants[i as usize].free_at <= self.current_cycle {
                // Set the worker as free.
                if (self.ants[i as usize].current_task != -1) {
                    let finished_task = self.ants[i as usize].current_task;

                    // there is a task to mark as completed so:
                    // Reduce the remaining counts for neighbors of the finished task.
                    // Remove the task node from the graph as well as its edges.

                    self.reduce_and_destroy(finished_task);
                    self.ants[i as usize].complete_task(finished_task, self.current_cycle);

                    //println!("Completed Task {} !", finished_task + 1);
                }
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
                // println!("Unlocked Task {} !", nb + 1);
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
    fn choose_task_randomly_weighted(&mut self, free_ant: i32) {
        // Collect available tasks and their corresponding visibility weights
        let mut candidate_tasks = Vec::new();
        let mut weights = Vec::new();

        for (i, &is_available) in self.available_tasks.iter().enumerate() {
            if is_available {
                let last_task = self.ants[free_ant as usize].last_task;
                let pherohormones_sum = self.pherohormones.borrow().pheromones_sum;

                let pheromone = if last_task != -1 {
                    self.pherohormones
                        .borrow()
                        .find_paths(last_task)
                        .iter()
                        .find(|path| path.task == i as i32)
                        .map(|path| path.weight)
                        .unwrap_or(0.0) // Default to 0.0 if no pheromone
                } else {
                    0.0
                };
                // Calculate the weight based on visibility and pheromone
                // the fucntion is visibility^alfa * pheromone^beta/ visibility_sum^alfa * pheromone_sum^beta
                let mut weight: f64 = 0.0;
                if pherohormones_sum > 0.0 {
                    weight = ((self.visibility_vec[i] as f64).powf(self.alfa)
                        * (pheromone as f64).powf(self.beta))
                        / (self.visibility_sum.powf(self.alfa) * pherohormones_sum.powf(self.beta));
                }

                if weight > self.max_weight {
                    self.max_weight = weight;
                }
                // println!(
                //     "Task {} has pheromone_sum {} and visibility {} the equation resul is {} * {} / {} * {}  = {}  + {}",
                //     i, pherohormones_sum, self.visibility_vec[i] ,
                //     (self.visibility_vec[i] as f64).powf(self.alfa)
                //     , (pheromone as f64).powf(self.beta),
                //     self.visibility_sum.powf(self.alfa)
                //     , pherohormones_sum.powf(self.beta) ,
                //     weight - self.base_chance,
                //     self.base_chance
                // );

                weights.push(weight + self.base_chance);

                candidate_tasks.push(i as i32);
            }
        }

        // Ensure there are available tasks to choose from
        if candidate_tasks.is_empty() {
            //println!("no tasks available , ant {} will wait", freeAnt);
            //do no more in this fucntion
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

            // Mark the task as no longer available
            self.available_tasks[*chosen_task as usize] = false;

            let free_at = self.current_cycle + self.costs_vec[*chosen_task as usize];
            let last_task = self.ants[free_ant as usize].last_task;
            self.ants[free_ant as usize].start_task(
                last_task as i32,
                *chosen_task as i32,
                free_at,
                &mut self.pherohormones.borrow_mut(),
                self.deposit_rate,
                self.current_cycle,
            );

            //println!("Ant {} has Started task {} ", freeAnt, chosen_task + 1);
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
