use std::{collections::HashMap, io::Error, str::FromStr};

use crate::modules::http::Method;

#[derive(Debug, Clone)]
pub struct Request { 
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub path: Vec<String>,
    pub method: Method,
    pub body: String,
}

impl Request {
    pub fn new(request: &str) -> Result<Self, Error> {
        
        let protocol = request.split(" ").nth(2).unwrap().to_string();
        let method = request.split(" ").nth(0).unwrap().to_string();
        let path = request.split(" ").nth(1).unwrap().to_string();
        
        let mut headers: HashMap<String, String> = HashMap::new();
        let params = HashMap::new();

        let mut path: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        path.retain(|x| !x.is_empty());
        
        let method = match Method::from_str(&method) {
            Err(_) => return Err(Error::new(std::io::ErrorKind::Other, "Method not supported")),
            Ok(method) => method,
        };

        if !protocol.starts_with("HTTP") {
            return Err(Error::new(std::io::ErrorKind::Other, "Protocol not supported"));
        }
        
        let lines: Vec<&str> = request.lines().skip(1).collect();
        
        for line in lines {
            if line.is_empty() { break; }
            let mut parts = line.split_whitespace();
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            headers.insert(key, value);
        };

        let body = request.split("\r\n\r\n").nth(1).unwrap().to_string();
        Ok(Self { headers, body, params, path, method  })
    }
}
