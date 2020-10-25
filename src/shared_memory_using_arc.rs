use std::sync::{Arc, Mutex};
use std::thread;

pub fn arc_impl() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    println!("counter : {}", *counter.lock().unwrap());
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter : {}", *counter.lock().unwrap());
}
