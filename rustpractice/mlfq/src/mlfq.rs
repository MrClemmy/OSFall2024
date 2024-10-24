// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    // in this case I had to make process mutable so that I could change
    // its priority level in the case that its out of range which I 
    // discovered through testing
    pub fn add_process(&mut self, mut process: Process) {
        // if the priority is too low just place in lowest priority queue
        if process.priority >= self.num_levels {
            process.priority = self.num_levels - 1;
        }
        self.queues[process.priority].push(process);
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // Plan: Check queue index, use time_quanta for that queue index
        // to update remaining time, total time, and current time
        // if remaining time is 0, remove the process, else lower priority

        // Make sure input is valid 
        if queue_index >= self.num_levels{
            println!("There is no such queue!");
            return;
        }

        // Make sure queue is not empty 
        if self.queues[queue_index].is_empty(){
            println!("There are no processes to run in this queue.");
            return;
        }

        // We need to get the time quanta for this level, im making it mutable because
        // I dont know if it needs to be or not but this makes me feel safe
        // if I dont make it mutable will it not be able to change for the next queue?
        // or are all executions completely independent of each other?
        // I'm going to make it immutable because I think I'm right

        let time_slice = self.time_quanta[queue_index];
        
        // need to collect the process we are about to update
        // it should be the process in the first position of that queue 
        // this line i tweaked by removing it from the queue as i was 
        // getting an error for double borrowing when i tried making a
        // reference to the process 
        let mut process = self.queues[queue_index].remove(0);

        // We need to know the execution time this is going to take
        // Which will be the min of time_quantum and remaining_time

        let execution_time = if process.remaining_time >= time_slice{
            time_slice
        }else{
            process.remaining_time
        };

        // Now we get to update all the time levels!

        process.total_executed_time += execution_time;
        process.remaining_time -= execution_time;
        self.current_time += execution_time;

        // Now we see if the process has completed and decide to
        // remove it or lower the priority level

        // if the process completes we remove it from the queue
        // we will always look at index 0

        // Then we need to see if the process is in the lowest priority queue
        // if it is, and hasn't completed we can leave it there for now
        // otherwise we add 1 to the queue level

        if process.remaining_time == 0{
            println!("Process {} has completed at {}", process.id, self.current_time);
        } else {
            if process.priority < self.num_levels - 1{
                // move to lower priority queue if not in lowest already
                process.priority += 1;
                self.queues[process.priority].push(process);
            } else {
                // if neither of the former cases are true we must
                // already be in the lowest queue, so we can push it back
                // to where it was without updating the queue index
                self.queues[queue_index].push(process);
            }
        }
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // TODO: Implement this function
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0

        // We don't need to boost those in queue 0 as they
        // are already in highest priorty
        for queue_index in 1..self.num_levels{
            // push until current queue is empty
            while !self.queues[queue_index].is_empty(){
                let mut process = self.queues[queue_index].remove(0);
                process.priority = 0;
                self.queues[0].push(process);
            }
        }
    }
        

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }

    #[test]
    fn test_changing_queues(){
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process {id: 1, priority: 0, remaining_time: 3, total_executed_time: 0});
        mlfq.queues[1].push(Process {id: 2, priority: 1, remaining_time: 5, total_executed_time: 0});
        mlfq.queues[2].push(Process {id: 3, priority: 2, remaining_time: 10, total_executed_time: 0});

        mlfq.execute_process(0);
        mlfq.execute_process(1);
        mlfq.execute_process(2);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 2);
    }

    // this test is to make sure that any process out of range ends up
    // in the lowest priority queue
    #[test]
    fn process_priority_range(){
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.add_process(Process{id: 1, priority: 2, remaining_time: 5, total_executed_time: 5});
        mlfq.add_process(Process{id: 2, priority: 5, remaining_time: 5, total_executed_time: 5});
        mlfq.add_process(Process{id: 3, priority: 100, remaining_time: 5, total_executed_time: 5});
        mlfq.add_process(Process{id: 4, priority: 1000000, remaining_time: 5, total_executed_time: 5});

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 4);
    }

    // I wanted to make sure I understood where each process was going after 
    // each function was called, this helped me understand a bit
    // for some reason I thought push() added an item to index 0 of a vector
    // not sure where I got that idea but after learning more I tweaked my test
    // and was able to pass everything
    #[test]
    fn altogether_now(){
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        let process1 = Process { id: 1, priority: 0, remaining_time: 1, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 6, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 12, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);

        mlfq.update_time(100);

        assert_eq!(mlfq.queues[0].len(), 3);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);

        mlfq.execute_process(0);
        
        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);

        mlfq.execute_process(1);
        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 0);

        mlfq.execute_process(1);

        assert_eq!(mlfq.queues[2].len(), 1);

        mlfq.execute_process(2);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);

    }
}