use crossbeam::channel::{Receiver, bounded};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Job {
    id: u64,
    payload: String,
}

fn worker(id: usize, receiver: Receiver<Job>) {
    loop {
        match receiver.recv() {
            Ok(job) => {
                println!("Worker {} processing job {}", id, job.id);

                // Simulate work
                thread::sleep(Duration::from_millis(50));
            }
            Err(_) => {
                println!("Worker {} shutting down", id);
                break;
            }
        }
    }
}

fn main() {
    let (sender, receiver) = bounded::<Job>(1000); // bounded = backpressure

    let num_workers = 4;
    let num_producers = 2;

    // Spawn workers
    let mut handles = vec![];
    for i in 0..num_workers {
        let r = receiver.clone();
        handles.push(thread::spawn(move || worker(i, r)));
    }

    // Spawn producers
    let mut producer_handles = vec![];
    for p in 0..num_producers {
        let s = sender.clone();
        producer_handles.push(thread::spawn(move || {
            for i in 0..5000 {
                let job = Job {
                    id: (p * 5000 + i) as u64,
                    payload: "data".into(),
                };

                s.send(job).unwrap();
            }
        }));
    }

    // Wait producers
    for h in producer_handles {
        h.join().unwrap();
    }

    // Drop sender → closes channel
    drop(sender);

    // Wait workers
    for h in handles {
        h.join().unwrap();
    }

    println!("All jobs processed.");
}
