#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashSet;
use std::net::{SocketAddr, TcpListener, TcpStream};

use rust_project::ThreadPool;
use std::io::prelude::*;
use std::ops::FnOnce;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Callback = fn(&mut TcpStream);

pub struct Server {
    address: SocketAddr,
    routes: Vec<Route>,
    workers: usize,
}

impl Server {
    pub fn new(address: SocketAddr, routes: Vec<Route>, workers: usize) -> Self {
        Self {
            address,
            routes,
            workers,
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        let pool = ThreadPool::new(self.workers);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];

            stream.read(&mut buffer).unwrap();
            let mut req = String::from_utf8_lossy(&mut buffer).to_string();
            let request = parse_request(&mut req).expect("could not parse the request");

            match request.method.to_lowercase().as_str() {
                "get" => {
                    println!("Get Method: {}", request.path);

                    /*  let data = self
                    .routes
                    .into_iter()
                    .position(|x| x.method == Method::GET); */
                }
                "post" => println!("Post Method: {}", request.path),
                "put" => println!("Put method: {}", request.path),
                "delete" => println!("Delete method: {}", request.path),
                _ => println!("Method not implemented"),
            }

            self.routes[0].execute_callback(&mut stream)
            // pool.execute::<_>(move || {});
        }

        println!("Shutting down.");
    }
}

struct Request {
    method: String,
    path: String,
}

fn parse_request(request: &mut String) -> Result<Request, ()> {
    let mut parts = request.split(" ");
    let method = match parts.next() {
        Some(method) => method.trim().to_string(),
        None => return Err(()),
    };
    let path = match parts.next() {
        Some(path) => path.trim().to_string(),
        None => return Err(()),
    };

    Ok(Request {
        method: method,
        path: path,
    })
}

type Job = Box<dyn Fn(&mut TcpStream) + Send + Sync + 'static>;

pub struct Route {
    pub method: Method,
    pub endpoint: String,
    pub callback: Callback,
    pub middleware: Option<Callback>,
}

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Route {
    pub fn new(
        method: Method,
        endpoint: String,
        callback: Callback,
        middleware: Option<Callback>,
    ) -> Self {
        Self {
            method,
            endpoint,
            callback,
            middleware,
        }
    }

    pub fn execute_callback(&self, stream: &mut TcpStream) {
        (self.callback)(stream)
    }
}
