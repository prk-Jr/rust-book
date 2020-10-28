#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashSet;
use std::net::TcpStream;

use std::ops::FnOnce;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
pub struct Config {
    routes: Vec<Route>,
}
type Job = Box<dyn Fn(&mut TcpStream) + Send + 'static>;
pub struct Route {
    pub method: Method,
    pub endpoint: String,
    pub callback: Job,
    pub middleware: Option<Job>,
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Route {
    pub fn new(method: Method, endpoint: String, callback: Job, middleware: Option<Job>) -> Self {
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
