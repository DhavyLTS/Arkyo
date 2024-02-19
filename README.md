# Arkyo

Arkyo is a simple HTTP server written in Rust, designed to handle incoming requests and provide appropriate responses based on defined routes and static file serving capabilities.

## Features

- **HTTP Server**: Arkyo listens for incoming TCP connections, parses HTTP requests, and dispatches them to corresponding handlers.
- **Routing**: Define routes with specific HTTP methods and corresponding handler functions.
- **Static File Serving**: Serve static files from a specified folder.
- **Error Handling**: Handle errors gracefully with appropriate HTTP status codes.

## Roadmap

- [ ] Dynamic Routes
- [ ] Multiform Support

## Usage

1. **Define Routes**: Add routes to the server instance using the `add_route` method.
```rust
    server.add_route("/example", Method::Get, handler_function);
```
2. **Set Static Folder**: Optionally, set a folder from which to serve static files using `static_folder` method.
```rust
    server.static_folder("public");
```
3. **Start Server**: Begin listening for incoming connections using `listen` method.
```rust
    server.listen();
```

## Example
```rust
    use arkyo::{Server, Method, Response, Request};

    fn main() {
        let mut server = Server::new("127.0.0.1:8080");
        // Define routes
        server.add_route("/", Method::Get, index_handler);
        server.add_route("/about", Method::Get, about_handler);
        // Set static folder
        server.static_folder("./static");
        // Start server
        server.listen();
    }

    fn index_handler(_: Request) -> Response {
        Response::new()
            .status(200)
            .body("Welcome to Arkyo!")
    }

    fn about_handler(_: Request) -> Response {
        Response::new()
            .status(200)
            .body("Arkyo - A simple HTTP server written in Rust.")
    }
```
    
## Installation
Add arkyo to your Cargo.toml dependencies:
```toml
    [dependencies]
    arkyo = "0.0.1"
```

## Contributing
- Found a bug? Please open an issue.
- Want to contribute? Fork the repository and submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

Arkyo - © 2024 DhavyLTS