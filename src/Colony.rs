use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error, Write};

use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use petgraph::Direction;

use crate::Utils::Utils;

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
    current_cycle: i32,
}

impl Colony {
    pub fn new(utils: Utils) -> Colony {
        Colony {
            n_ants: 0,
            ants: Vec::new(),
            // the utils struct is passed by value and we just need to clone it because threre wuill be a lot of colonies and we dont want to share the same utils struct
            //utils: utils.clone(),
            di_graph: utils.di_graph.clone(),
            n_tasks: utils.n_tasks,
            remaining_vec: utils.remaining_vec.clone(),
            current_cycle: 0,
        }
    }
    pub fn work(&mut self) {}

    fn check_tasks_completion(&mut self) {
        for i in 0..self.n_ants {
            if self.ants[i as usize].free_at >= self.current_cycle {
                self.ants[i as usize].current_task = -1;
                self.ants[i as usize].free_at = -1;
                //Reduce neighboors
                // if 0 remaining put in the available tasks
            }
        }
    }

    fn reduce_neighboors(&mut self, task: i32) {}
    fn check_available_tasks(&mut self) {}
    fn choose_task_randomly_weighted(&mut self) {}
}
