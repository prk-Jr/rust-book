#![allow(dead_code)]
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
use serde::{Deserialize, Serialize};

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

fn main() {
    let _count = 0;
    let routes = vec![
        Route::new(Method::Get, "/", my_res),
        Route::new(Method::Post, "/<String:name>/<int:age>", my_res_post),
    ];
    // let _pool = ThreadPool::new(50);

    let mut server = Server::new(String::from("192.168.1.22:8000"), routes, 10);
    server.run();

    println!("Shutting down.");
}

fn my_res(req: &Request) -> Response {
    let age = req.get::<i32>("age");
    println!("age: {:?}, json: {:?}", age, req.get_json());
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
#[derive(Serialize, Deserialize, Debug)]

struct Name {
    name: String,
}

fn my_res_post(req: &Request) -> Response {
    let _age = req.get::<i32>("age");

    // println!("age: {}, json :{:?} ", age, req.get_header("token"));
    let mut response = Response::new();

    let mut res: Vec<String> = Vec::new();
    for i in 0..100000 {
        let var = format!("{}:{}", i, "Hello World");
        res.push(var);
    }

    response.set_body(format!("{:?}", res));

    response.set_status(200);

    response
}

fn not_found(_req: &Request) -> Response {
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
