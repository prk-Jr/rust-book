use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::*;
fn main() {
    println!("Hello, world!");
    let mut count = 0;
    let server = TcpListener::bind("127.0.0.1:4080").unwrap();
    for stream in server.incoming() {
        count += 1;
        println!("hello {}", count);
        match stream {
            Ok(mut data) => {
                {
                    thread::spawn(move || handle_stream(&mut data));
                };
            }
            Err(data) => eprint!("Some error occured {:?} at {:?}", data, SystemTime::now()),
        }
    }
}

fn handle_stream(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let mut res: Vec<String> = Vec::new();
    for i in 0..10000 {
        let var = format!("{}:{}", i, "Hello World");
        res.push(var);
    }

    stream
        .write(format!("{}{:?}", response, res).as_bytes())
        .unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
