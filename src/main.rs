use std::sync::mpsc;
use std::{thread, time};
mod my_threads;
mod shared_memory;
mod shared_memory_using_arc;

fn main() {
    let t1 = time::SystemTime::now();
    shared_memory_using_arc::arc_impl();
    my_threads::create_my_threads_with_same_rx();
    shared_memory::use_mutex();
    let (tx, rx) = mpsc::channel();
    let (ntx, nrx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(1000));
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    thread::spawn(move || {
        let val = String::from("hello");
        ntx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    let receivedn = nrx.recv().unwrap();
    println!("n Got: {}", receivedn);
    println!("Got: {}", received);
    println!(
        "time since: {:?}",
        time::SystemTime::now().duration_since(t1)
    );
}
