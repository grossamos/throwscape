use std::thread;
use std::sync::mpsc;

use super::scheduler::{Job, Scheduler};

pub struct ThreadPool {
    _workers: Vec<Worker>,
    scheduler: Scheduler,
}

struct Worker {
    pub _id: u32,
}

impl ThreadPool {
    pub fn new(threads: u32) -> ThreadPool {
        if threads == 0 {
            panic!("Invalid thread count provided");
        }

        let (waiting_threads_tx, waiting_threads_rx) = mpsc::channel::<u32>();

        let mut workers = Vec::with_capacity(threads as usize);
        let mut outgoing_job_senders = Vec::with_capacity(threads as usize);

        for id in 0..threads {
            let current_waiting_sender = waiting_threads_tx.clone();
            let (current_job_tx, current_job_rx) = mpsc::channel::<Job>();
            outgoing_job_senders.push(current_job_tx);

            let current_worker = Worker {
                _id: id,
            };

            thread::spawn(move || {
                loop {
                    current_waiting_sender.send(id).unwrap();

                    let mut job = current_job_rx.recv().unwrap();

                    job();
                }
            });

            workers.push(current_worker);
        }
        

        ThreadPool { 
            _workers: workers,
            scheduler: Scheduler::new(waiting_threads_rx, outgoing_job_senders), 
        }
        
    }

    pub fn handle_job(&self, job: Job) {
        self.scheduler.assign(job);
    }
}

