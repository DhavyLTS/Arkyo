use std::collections::HashMap;
use chrono::Utc;

use super::http::*;

#[derive(Debug)]
pub struct Response {
    headers: Option<HashMap<String, String>>,
    status: Option<Status>,
    body: Option<String>,
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response = String::new();
        
        let body_binding: String = String::new();
        let headers_binding: HashMap<String, String> = HashMap::new();

        let body = self.body.as_ref().unwrap_or(&body_binding);
        let headers = self.headers.as_ref().unwrap_or(&headers_binding);
        let status = self.status.as_ref().unwrap_or(&Status::Ok).to_string();

        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT");

        response += format!("HTTP/1.1 {} \r\n", status).as_str();
        response += format!("Date: {}\r\n", date).as_str();
        response += format!("Server: Arkyo/0.0.4\r\n").as_str();

        for (key, value) in headers.iter() {
            response += format!("{}: {}\r\n", key, value).as_str();
        }
        
        response += format!("\r\n{}", body).as_str();

        response
    }
}

impl Response {
    pub fn new() -> Self { Self { headers: None, status: None, body: None } }
    pub fn headers (mut self, headers: HashMap<String, String>) -> Self { self.headers = Some(headers); self }
    pub fn status (mut self, status: Status) -> Self { self.status = Some(status); self }
    pub fn body (mut self, body: &str) -> Self { self.body = Some(body.to_string()); self }
}
