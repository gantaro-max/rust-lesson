use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    let counter = Arc::new(Mutex::new(0));
}
