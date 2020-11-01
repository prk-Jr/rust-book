use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
    NoImpl,
}

#[derive(Debug)]
pub enum RequestError {
    JsonStrError(serde_json::Error),
    StrCopyError(std::string::FromUtf8Error),
}

impl From<serde_json::Error> for RequestError {
    fn from(err: serde_json::Error) -> RequestError {
        RequestError::JsonStrError(err)
    }
}

impl From<std::string::FromUtf8Error> for RequestError {
    fn from(err: std::string::FromUtf8Error) -> RequestError {
        RequestError::StrCopyError(err)
    }
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RequestError::JsonStrError(err) => write!(f, "JSON error: {}", err),
            RequestError::StrCopyError(err) => write!(f, "UTF-8 error: {}", err),
        }
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::JsonStrError(err) => Some(err),
            RequestError::StrCopyError(err) => Some(err),
        }
    }
}

pub trait FromUri {
    /// A function to parse a string into the correct type.
    fn from_uri(data: &str) -> Self;
}

impl FromUri for String {
    fn from_uri(data: &str) -> String {
        String::from(data)
    }
}

impl FromUri for i32 {
    fn from_uri(data: &str) -> i32 {
        data.parse::<i32>()
            .expect("matched integer can't be parsed")
    }
}

impl FromUri for u32 {
    fn from_uri(data: &str) -> u32 {
        data.parse::<u32>()
            .expect("matched integer can't be parsed")
    }
}

impl FromUri for f32 {
    fn from_uri(data: &str) -> f32 {
        data.parse::<f32>().expect("matched float can't be parsed")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub payload: Vec<u8>,
    pub params: HashMap<String, String>,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: Method::NoImpl,
            path: String::new(),
            headers: HashMap::new(),
            params: HashMap::new(),
            payload: Vec::with_capacity(2048),
        }
    }

    pub fn get_header(&self, name: &str) -> Option<String> {
        let key = String::from(name);

        match self.headers.get(&key) {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    pub fn get<T: FromUri>(&self, name: &str) -> Option<T> {
        match self.params.contains_key(name) {
            false => None::<T>,
            true => Some(FromUri::from_uri(&self.params[name])),
        }
    }

    pub fn get_json(&self) -> Result<serde_json::Value, RequestError> {
        let payload = String::from_utf8_lossy(&self.payload);
        let data = serde_json::from_str(&payload)?;

        Ok(data)
    }

    pub fn get_json_obj<T>(&self) -> Result<T, RequestError>
    where
        T: DeserializeOwned,
    {
        let payload = String::from_utf8(self.payload.clone())?;
        let data = serde_json::from_str(&payload)?;

        Ok(data)
    }

    fn parse(&mut self, rqstr: &str) {
        let mut buf: Vec<&str> = rqstr.splitn(2, "\r\n").collect();
        let ask: Vec<&str> = buf[0].splitn(3, ' ').collect();

        self.method = match ask[0] {
            "GET" => Method::Get,
            "PUT" | "PATCH" => Method::Put,
            "POST" => Method::Post,
            "DELETE" => Method::Delete,
            _ => Method::NoImpl,
        };

        self.path = String::from(ask[1]);
        if ask.len() == 2 {
            loop {
                buf = buf[1].splitn(2, "\r\n").collect();

                if buf[0] == "" {
                    if buf.len() == 1 || buf[1] == "" {
                        // no payload
                        break;
                    }

                    self.payload.extend(buf[1].as_bytes());
                    break;
                }

                let hdr: Vec<&str> = buf[0].splitn(2, ": ").collect();

                if hdr.len() == 2 {
                    self.headers
                        .insert(String::from(hdr[0]), String::from(hdr[1]));
                }
            }
        }
    }

    pub fn from_str(rqstr: &str) -> Result<Self, Err> {
        let mut req = Request::new();
        req.parse(rqstr);
        Ok(req)
    }
}
type Err = RequestError;

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

impl std::str::FromStr for Request {
    type Err = RequestError;

    /// Create a Request from an HTTP request string.
    fn from_str(rqstr: &str) -> Result<Self, Self::Err> {
        let mut req = Request::new();
        req.parse(rqstr);
        Ok(req)
    }
}
