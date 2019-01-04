// threads1.rs

use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut stt = status_shared.lock().unwrap();
            stt.jobs_completed += 1;
        }
    });
    let status_main = status.clone();
    let mut status_jobs = {
        let stt = status_main.lock().unwrap();
        stt.jobs_completed
    };
    while status_jobs < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
        status_jobs = {
            let stt = status_main.lock().unwrap();
            stt.jobs_completed
        };
    }
}