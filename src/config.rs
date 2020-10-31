#![allow(dead_code)]
#![allow(unused_imports)]

use rust_project::response::*;
use rust_project::route::*;
use rust_project::threadpool::*;
use rust_project::{request::*, route};
use std::collections::HashSet;
use std::net::{SocketAddr, TcpListener, TcpStream};

use std::io::prelude::*;
use std::ops::FnOnce;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Callback = fn(&mut TcpStream);

pub struct Server {
    address: String,
    routes: Vec<Route>,
    workers: usize,
}

impl Server {
    pub fn new(address: String, routes: Vec<Route>, workers: usize) -> Self {
        Self {
            address,
            routes,
            workers,
        }
    }

    pub fn run(&mut self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        println!(
            "Server Running at: {}",
            match self.address.contains("http") {
                false => format!("http://{}", self.address),
                _ => format!("{}", self.address),
            }
        );

        let _pool = ThreadPool::new(self.workers);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let mut buffer = [0; 1024];

            stream.read(&mut buffer).unwrap();
            let req = String::from_utf8_lossy(&mut buffer).to_string();

            stream
                .write(&self.handle_request(&req).send())
                .expect("Error occured while repondinf to stream");
            stream
                .flush()
                .expect("Error occured while reponding to stream");
        }
    }

    fn handle_request(&mut self, rqstr: &str) -> Response {
        let mut req = Request::from_str(&rqstr).unwrap();

        let mut found = false;
        let mut res = Response::new();
        for i in 0..self.routes.len() {
            if self.routes[i].is_match(&req) {
                let handler = self.routes[i].handler;
                req.params = self.routes[i].parse(&req.path);
                res = handler(&req);

                found = true;
                break;
            }
        }

        if found {
            res.set_status(200);
            res
        } else {
            res.set_status(404);

            res
        }
    }
}

type Job = Box<dyn Fn(&mut TcpStream) + Send + Sync + 'static>;

/* pub struct Route {
    pub method: Method,
    pub endpoint: String,
    pub callback: Callback,
    pub middleware: Option<Callback>,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
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
 */
