use std::sync::mpsc;
use std::thread;
use std::time;

pub fn create_my_threads_with_same_rx() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![String::from("1"), String::from("2"), String::from("3")];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
    thread::spawn(move || {
        let vals = vec![String::from("a1")];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    for item in rx {
        println!("Got : {}", item);
    }
}
