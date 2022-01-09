use std::{sync::mpsc::{Receiver, Sender, self}, thread};

use crate::configuration::Config;

pub struct Scheduler {
    job_assignment_sender: Sender<Job>,
}

impl Scheduler {
    pub fn new(waiting_threads: Receiver<u32>, outgoing_job_senders: Vec<Sender<Job>>) -> Scheduler {
        let (job_assignment_sender, job_assignment_reciever) = mpsc::channel();

        thread::spawn(move || {
            loop {
                // main scheduling logic: just have threads pull for tasks
                let avail_worker_id = waiting_threads.recv().unwrap();

                let job = job_assignment_reciever.recv().unwrap();

                outgoing_job_senders
                    .get(avail_worker_id as usize).unwrap()
                    .send(job).unwrap();
            }
        });
        Scheduler {
            job_assignment_sender,
        }
    }
    
    pub fn assign(&self, job: Job) {
        self.job_assignment_sender.send(job).unwrap();
    }
}

pub type Job = Box<dyn FnMut(&Config) + Send>;

