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

        pool.execute(move || handle_stream(&mut stream));
    }

    println!("Shutting down.");
}

fn handle_stream(mut stream: &mut TcpStream) {
    let routes: Vec<Route> = vec![
        Route::new(Method::GET, String::from("/hello"), my_res, None),
        Route::new(Method::POST, String::from("/hello/prk"), my_res, None),
    ];
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let mut req = String::from_utf8_lossy(&mut buffer).to_string();
    let request = parse_request(&mut req).expect("could not parse the request");

    match request.method.to_lowercase().as_str() {
        "get" => {
            println!("Get Method: {}", request.path);

            let mut found: bool = false;
            for (_, route) in routes.into_iter().enumerate() {
                if route.method == Method::GET && route.endpoint == request.path.to_string() {
                    found = true;
                    route.execute_callback(&mut stream);
                    break;
                }
            }
            if found == false {
                not_found(&mut stream);
            }
        }
        "post" => {
            println!("Post Method: {}", request.path);
            let mut found: bool = false;
            for (_, route) in routes.into_iter().enumerate() {
                if route.method == Method::POST && route.endpoint == request.path.to_string() {
                    found = true;
                    route.execute_callback(&mut stream);
                    break;
                }
            }
            if found == false {
                not_found(&mut stream);
            }
        }
        "put" => {
            println!("Put method: {}", request.path);
            let mut found: bool = false;
            for (_, route) in routes.into_iter().enumerate() {
                if route.method == Method::PUT && route.endpoint == request.path.to_string() {
                    found = true;
                    route.execute_callback(&mut stream);
                    break;
                }
            }
            if found == false {
                not_found(&mut stream);
            }
        }
        "delete" => {
            println!("Delete method: {}", request.path);
            let mut found: bool = false;
            for (_, route) in routes.into_iter().enumerate() {
                if route.method == Method::DELETE && route.endpoint == request.path.to_string() {
                    found = true;
                    route.execute_callback(&mut stream);
                    break;
                }
            }
            if found == false {
                not_found(&mut stream);
            }
        }
        _ => {
            println!("Method not implemented");

            not_found(&mut stream);
        }
    }
}

fn my_res(stream: &mut TcpStream) {
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
}

fn not_found(stream: &mut TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    let mut res: Vec<String> = Vec::new();
    for i in 0..100 {
        let var = format!("{}:{}", i, "Not Found");
        res.push(var);
    }

    stream
        .write(format!("{}{:?}", response, res).as_bytes())
        .unwrap();
    stream.flush().unwrap();
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
