use std::net::TcpListener;


use super::middleware::Middleware;
use super::connection::handler;
use super::route::Route;
use super::prelude::*;

pub struct Server { 
    static_folder: Option<String>,
    middleware: Vec<Middleware>, 
    routes: Vec<Route>,
}

impl Server {

    pub fn new() -> Server {
        Server { routes: Vec::new(), middleware: Vec::new(), static_folder: None }
    }

    pub fn add_route(&mut self, path: &str, method: Method , handler: fn(Request) -> Response) {
        self.routes.push(Route::new(path.to_string(), method, handler));
    }

    pub fn add_middleware(&mut self, path: &str, handler: fn(Request) -> Result<Request, Response>) {
        self.middleware.push(Middleware::new(path.to_string(), handler))
    }

    pub fn static_folder(&mut self, path: &str) {
        self.static_folder = Some(path.to_string());
    }

    pub fn listen (&self, addr: &str) {
        let listener = match TcpListener::bind(addr) {
            Err(err) => panic!("Couln't bind to address: {:?}", err),
            Ok(listener) => listener,
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) =>  {
                    let routes = self.routes.clone();
                    let middlewares = self.middleware.clone();
                    let static_folder = self.static_folder.clone();
                    std::thread::spawn(move || handler(stream, middlewares, routes, static_folder));
                },
                Err(err) => return println!("Connection failed: {:?}", err), 
            }   
        }
    }
}   
