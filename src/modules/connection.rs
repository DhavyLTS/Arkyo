use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::fs::File;

use super::middleware::Middleware;
use super::route::Route;
use super::prelude::*;

pub fn handler(mut stream: TcpStream, middlewares: Vec<Middleware>, routes: Vec<Route>, static_folder: Option<String>) {
    let mut buf = [0; 1024];

    let request_str = match stream.read(&mut buf) {
        Err(_) => {
            let response = Response::new().status(Status::InternalServerError);
            return stream.write_all(response.to_string().as_bytes()).unwrap()
        },
        Ok(size) => String::from_utf8_lossy(&buf[..size]),
    };

    let mut request = match Request::new(&request_str) {
        Err(_) => {
            let response = Response::new().status(Status::BadRequest);
            return stream.write_all(response.to_string().as_bytes()).unwrap()
        }, 
        Ok(request) => request,
    };

    if let Some(static_folder) = &static_folder {
        let path = String::new() + &static_folder + "/" + &request.path.join("/");

        if Path::new(&path).exists() {
            let mut file_string = String::new();
            let mut file = match File::open(&path) {
                Err(_) => {
                    let response = Response::new().status(Status::InternalServerError);
                    return stream.write_all(response.to_string().as_bytes()).unwrap()
                },
                Ok(file) => file,
            };

            file.read_to_string(&mut file_string).unwrap();

            let response = Response::new()
                .status(Status::Ok)
                .body(&file_string);

            return stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }

    let mut opt_route: Option<&Route> = None;

    for route in routes.iter() {
        if route.path.len() != request.path.len() || route.method != request.method { continue; }

        let mut path: Vec<String> = Vec::new();

        for (request_path, route_path) in request.path.iter().zip(route.path.iter()) {
            if !route_path.starts_with(":") && request_path != route_path { break; }

            if route_path.starts_with(":") {
                let key = route_path[1..].to_string();
                let value = request_path.to_string();
                request.params.insert(key, value);
                path.push(route_path.to_string());
                continue;
            } 

            path.push(route_path.to_string());
        }

        if path == route.path { 
            opt_route = Some(route); 
            break; 
        }

    }

    let mut response: Option<Response> = None;

    if let Some(route) = opt_route {
        for middleware in middlewares {
            if route.path.starts_with(&middleware.path) {
                match middleware.handle(request.clone()) {
                    Err(_response) => { response = Some(_response); break; }
                    Ok(_request) => { request = _request; },
                }
            }
        }
        if response.is_none() { response = Some(route.handle(request)); }
    } else { response = Some(Response::new().status(Status::NotFound)); }

    stream.write_all(response.unwrap().to_string().as_bytes()).unwrap();
}
