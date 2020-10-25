use std::sync::mpsc;
use std::{thread, time};
mod my_threads;

fn main() {
    let t1 = time::SystemTime::now();
    my_threads::create_my_threads_with_same_rx();
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
