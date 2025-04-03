use crate::pherohormones::{self, Pherohormones};
#[derive(Clone)]
pub(crate) struct WorkerAnt {
    // the identifyer of the worker is the index of the array
    pub current_task: i32,
    pub last_task: i32,
    pub free_at: i32,
    pub task_history: Vec<(i32, i32, i32)>,
}
impl WorkerAnt {
    pub fn new(n_tasks: i32) -> WorkerAnt {
        WorkerAnt {
            // the values for the current task are -1 which means free and  any other value higher than -1 means busy
            current_task: -1, //free
            last_task: -1,
            // the value for free_at  means the cycle that the worker will be free or better iteration
            free_at: -1,
            task_history: vec![(0, 0, 0); n_tasks as usize],
        }
    }
    pub fn start_task(
        &mut self,
        last_task: i32,
        chosen_task: i32,
        free_at: i32,
        pherohormones: &mut Pherohormones,
        deposit_rate: f64,
        current_cycle: i32,
    ) {
        self.current_task = chosen_task as i32;
        // Set the time when the ant will be free again; adjust as necessary
        self.free_at = free_at;
        if self.last_task != -1 {
            pherohormones.deposit_pherohormones(last_task, chosen_task, deposit_rate);
        }
        self.task_history[chosen_task as usize] = (chosen_task, current_cycle, -1);

        //pherohormones.deposit_pherohormones(self.last_task, chosen_task, deposit_rate);
    }
    pub fn complete_task(&mut self, finished_task: i32, current_cycle: i32) {
        self.current_task = -1;
        self.free_at = -1;
        self.last_task = finished_task;
        self.task_history[finished_task as usize] = (
            finished_task,
            self.task_history[finished_task as usize].1,
            current_cycle,
        );
    }
}
