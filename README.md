🦀 oxide-http1.1
RUST + OXYGEN = OXIDE 😁
A fully functional multithreaded HTTP/1.1 server built from scratch in Rust.
It supports routing, persistent (keep-alive) connections, gzip compression, and graceful connection handling — all powered by a custom thread pool for concurrency.
Designed to be lightweight, educational, and modular for extending with new features.

⚡ Features
Multithreaded Architecture — custom thread pool for handling multiple clients concurrently.
Persistent Connections — supports HTTP/1.1 keep-alive for efficient reuse of TCP sessions.
Routing System — simple router for registering and handling routes (GET, POST, PUT, DELETE).
Gzip Compression — automatically compresses responses when clients support it.
Modular Design — clean separation between HTTP parsing, routing, and server layers.
Graceful Connection Closure — properly closes connections when clients disconnect or send Connection: close.
Fully Compatible with curl — tested with real HTTP clients and tools.

📁 Project Structure
src/
├── http/
│   ├── method.rs        # Defines HTTP methods (GET, POST, etc.)
│   ├── request.rs       # Parses raw HTTP requests
│   ├── response.rs      # Builds HTTP responses
│   ├── status.rs        # HTTP status codes
│   ├── utils.rs         # Helper utilities
│   └── mod.rs
├── router/
│   └── mod.rs           # Route registration and dispatch
├── server/
│   ├── handler.rs       # Handles client connections and request loops
│   ├── response.rs      # Response sending utilities
│   ├── errors.rs        # Server-level error handling
│   └── mod.rs
├── threadpool/
│   └── mod.rs           # Custom thread pool for concurrency
├── main.rs              # Server entry point
└── lib.rs               # Library exports

🛠 How It Works
Listens for incoming TCP connections.
Each connection is handled by a worker thread from the pool.
Supports persistent connections (multiple HTTP requests per TCP session).
Routes are matched and executed via a simple router system.
Responses are optionally gzip-compressed if the client supports it.
Connections are gracefully closed when requested or on timeout.

🚀 Run Locally
# Clone the repo
git clone https://github.com/Yeabsirashimelis/oxide-http1.1.git
cd oxide-http1.1

# Run the server
cargo run

Test with curl

GET requests
curl -v http://localhost:4221/
curl -v http://localhost:4221/about
curl -v http://localhost:4221/user/42


POST, PUT, DELETE requests
curl -X POST http://localhost:4221/user -d "name=dude-finetuner"
curl -X PUT http://localhost:4221/user/42 -d "name=dude-bsre"
curl -X DELETE http://localhost:4221/user/42


Gzip compression
curl -v --compressed http://localhost:4221/

# oxide-http1.1

RUST + OXYGEN = OXIDE

A small, educational, multithreaded HTTP/1.1 server written from scratch in Rust. It implements routing, persistent (keep-alive) connections, gzip compression, graceful connection handling, and a custom thread pool for concurrency. The project is designed to be lightweight and easy to extend.

## Features

- Multithreaded architecture using a custom thread pool
- Persistent connections (HTTP/1.1 keep-alive)
- Simple router with support for GET, POST, PUT, DELETE
- Optional gzip compression for responses when supported by the client
- Clean, modular code separating HTTP parsing, routing, and server logic
- Graceful connection closure and timeout handling
- Tested with common tools such as curl

## Project structure

```
src/
├── http/
│   ├── method.rs        # HTTP methods (GET, POST, etc.)
│   ├── request.rs       # HTTP request parsing
│   ├── response.rs      # HTTP response building
│   ├── status.rs        # Status codes
│   ├── utils.rs         # Helper utilities
│   └── mod.rs
├── router/
│   └── mod.rs           # Route registration and dispatch
├── server/
│   ├── handler.rs       # Client connection handling
│   ├── response.rs      # Response sending utilities
│   ├── errors.rs        # Server error types
│   └── mod.rs
├── threadpool/
│   └── mod.rs           # Custom thread pool
├── main.rs              # Server entry point
└── lib.rs               # Library exports
```

## How it works

1. Listen for incoming TCP connections on a configured port.
2. Each connection is handed to a worker from the thread pool.
3. The server parses HTTP/1.1 requests, matches routes, and generates responses.
4. Responses may be gzip-compressed when the client accepts it.
5. Connections are kept alive when requested and closed gracefully on timeout or client request.

## Run locally

Clone the repository and run with Cargo:

```powershell
git clone https://github.com/Yeabsirashimelis/oxide-http1.1.git
cd oxide-http1.1
cargo run
```

The server listens on the configured port (for example, 4221 by default in examples). Use curl or your browser to test endpoints.

### Example curl commands

GET requests

```powershell
curl -v http://localhost:4221/
curl -v http://localhost:4221/about
curl -v http://localhost:4221/user/42
```

POST, PUT, DELETE

```powershell
curl -X POST http://localhost:4221/user -d "name=dude-finetuner"
curl -X PUT http://localhost:4221/user/42 -d "name=dude-bsre"
curl -X DELETE http://localhost:4221/user/42
```

Gzip compression

```powershell
curl -v --compressed http://localhost:4221/
```

Persistent connections (HTTP/1.1)

```powershell
curl -v --http1.1 http://localhost:4221/
```

## Learning goals

This project demonstrates:

- Low-level TCP handling in Rust
- HTTP request/response parsing
- Designing a thread pool for concurrency
- Connection persistence and graceful shutdown
- Compression and header management
- Simple routing with dynamic parameters

## Future improvements

- Add middleware support and static file serving
- Implement request pipelining and async I/O (e.g. using Tokio)
- Improve router capabilities (better parameter parsing, middleware, etc.)

## Author

Yeabsira Shimelis

Built with ❤️ and Rust 🦀

- Low-level TCP handling in Rust
- HTTP request/response parsing
- Designing a thread pool for concurrency
- Connection persistence and graceful shutdown
- Compression and header management
- Simple routing with dynamic parameters

## Future improvements

- Add middleware support and static file serving
- Implement request pipelining and async I/O (e.g. using Tokio)
- Improve router capabilities (better parameter parsing, middleware, etc.)

## Author

Yeabsira Shimelis

Built with ❤️ and Rust 🦀
