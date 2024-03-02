use super::prelude::*;

#[derive(Debug, Clone)]
pub struct Route {
    handler: fn(Request) -> Response,
    pub path: Vec<String>,
    pub method: Method,
}

impl Route {
    pub fn new(path: String, method: Method, handler: fn(Request) -> Response) -> Route {
        let mut path: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        path.retain(|x| !x.is_empty());        
        Route { path, method, handler }
    }

    pub fn handle(&self, request: Request ) -> Response {
        (self.handler)(request)
    }
}
