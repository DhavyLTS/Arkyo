use super::response::Response;
use super::request::Request;

#[derive(Debug, Clone)]
pub struct Middleware {
    handler: fn(Request) -> Result<Request,Response>,
    pub path: Vec<String>
}

impl Middleware {

    pub fn new( path: String, handler: fn(Request) -> Result<Request, Response>) -> Self {
        let mut path: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        path.retain(|s| !s.is_empty());
        Self { handler, path }
    } 

    pub fn handle(&self, request: Request) -> Result<Request, Response> {
        (self.handler)(request)
    }

}
