#![allow(unused_imports)]

mod config;

mod request;
mod response;

mod utils;
use config::*;
use route::*;
use rust_project::request::*;
use rust_project::response::*;
use rust_project::route;
use rust_project::threadpool::*;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
fn main() {
    let _count = 0;
    let routes = vec![Route::new(Method::Get, "/hello", my_res)];
    let pool = ThreadPool::new(50);

    let mut server = Server::new(String::from("127.0.0.1:7878"), routes, 10);

    pool.execute(move || server.run());

    println!("Shutting down.");
}

fn handle_request(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let req = String::from_utf8_lossy(&mut buffer).to_string();
    let routes = vec![Route::new(Method::Get, "/hello", my_res)];

    let mut req = Request::from_str(&req).unwrap();

    let mut response: Vec<u8> = Vec::new();

    for route in routes {
        println!("is {}", route.is_match(&req));
        if route.is_match(&req) {
            req.params = route.parse(&req.path);

            let handler = route.handler;
            let res = handler(&req);
            response = res.send();
            break;
        }
    }

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

fn my_res(req: &Request) -> Response {
    let mut response = Response::new();

    let mut res: Vec<String> = Vec::new();
    for i in 0..100 {
        let var = format!("{}:{}", i, "Hello World");
        res.push(var);
    }

    response.set_body(format!("{:?}", res));

    response.set_status(200);

    response
}

fn not_found(req: &Request) -> Response {
    let mut response = Response::new();

    let mut res: Vec<String> = Vec::new();
    for i in 0..100 {
        let var = format!("{}:{}", i, "Not found");
        res.push(var);
    }
    // response.set_body(res);
    response.set_status(200);

    response
}
