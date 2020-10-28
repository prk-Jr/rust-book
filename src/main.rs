#![allow(unused_imports)]

use rust_project::ThreadPool;
mod config;
use config::{Config, Method, Route};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    println!("Hello, world!");
    let mut count = 0;
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        count += 1;
        let route = Route::new(
            Method::GET,
            String::from("/hello"),
            Box::new(handle_stream),
            None,
        );
        println!("Total requests {}", count);
        let mut stream = stream.unwrap();

        pool.execute(move || {
            route.execute_callback(&mut stream);
        });
    }

    println!("Shutting down.");
}

fn handle_stream(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    let mut res: Vec<String> = Vec::new();
    for i in 0..1000000 {
        let var = format!("{}:{}", i, "Hello World");
        res.push(var);
    }

    stream
        .write(format!("{}{:?}", response, res).as_bytes())
        .unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
