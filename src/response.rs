#![allow(dead_code)]
#![allow(unused_imports)]
use serde::Serialize;
use serde_json;
use std::collections::BTreeMap;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub trait ToOutput {
    fn to_output(&self) -> &[u8];
}

impl ToOutput for str {
    fn to_output(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ToOutput for &'static str {
    fn to_output(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ToOutput for String {
    fn to_output(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ToOutput for Vec<u8> {
    fn to_output(&self) -> &[u8] {
        self.as_slice()
    }
}

#[derive(Debug, Default)]
pub struct Response {
    status: u16,
    msg: String,
    content_type: String,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: 200,
            msg: String::from("OK"),
            content_type: String::from("text/plain"),
            headers: BTreeMap::new(),
            body: Vec::new(),
        }
    }

    pub fn set_content_type(&mut self, ctype: &str) {
        self.content_type = String::from(ctype);
    }

    pub fn set_body<T: ToOutput>(&mut self, body: T) {
        self.body.extend(body.to_output().iter());
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
        self.msg = Response::get_http_message(status);
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        if !self.headers.contains_key(key) {
            self.headers.insert(String::from(key), String::from(value));
        }
    }

    pub fn to_json<T: Serialize>(data: &T) -> Response {
        let mut res = Response::new();
        res.set_content_type("application/json");
        res.set_body(serde_json::to_string(data).unwrap());
        res
    }

    pub fn send(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::with_capacity(self.body.len() + 500);
        let mut inter = String::new();

        inter.push_str(&format!(
            "HTTP/1.1 {} {}\r\n",
            self.status, self.content_type
        ));

        for (key, value) in &self.headers {
            inter.push_str(&format!("{}: {}\r\n", key, value));
        }

        inter.push_str(&format!("Content-Type: {}\r\n", self.content_type));
        inter.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        inter.push_str("\r\n");

        output.extend(inter.as_bytes());
        output.extend(self.body.iter());

        output
    }

    fn get_http_message(status: u16) -> String {
        let msg = match status {
            100 => "Continue",
            101 => "Switching Protocols",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            305 => "Use Proxy",
            307 => "Temporary Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Time Out",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Request Entity Too Large",
            414 => "Request-URI Too Large",
            415 => "Unsupported Media Type",
            416 => "Requested Range Not Satisfiable",
            417 => "Expectation Failed",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Time-out",
            505 => "HTTP Version Not Supported",
            _ => "OK",
        };

        String::from(msg)
    }
}
