use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn run() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..5 {
        let counter_thread = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            *counter_thread.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("カウンター数: {}", counter.lock().unwrap());
}
