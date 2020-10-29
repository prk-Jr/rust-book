#![allow(unused_imports)]

use rust_project::ThreadPool;
mod config;
use config::{Method, Route, Server};
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
fn main() {
    println!("Hello, world!");
    let _count = 0;
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let route = Route::new(Method::GET, String::from("/hello"), handle_stream, None);

        pool.execute(move || {
            route.execute_callback(&mut stream);
        });
    }

    /*  let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);

    let routes: Vec<Route> = vec![Route::new(
        Method::GET,
        String::from("/hello"),
        Box::new(handle_stream),
        None,
    )];
    let server = Server::new(addr, routes, 10);

    server.run(); */

    println!("Shutting down.");
}

fn handle_stream(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    let mut res: Vec<String> = Vec::new();
    for i in 0..100 {
        let var = format!("{}:{}", i, "Hello World");
        res.push(var);
    }

    stream
        .write(format!("{}{:?}", response, res).as_bytes())
        .unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
