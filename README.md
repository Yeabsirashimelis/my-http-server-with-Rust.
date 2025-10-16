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


Persistent connections
curl -v --http1.1 http://localhost:4221/

🎯 Learning Goals
This project demonstrates:
Low-level TCP handling in Rust
HTTP request/response parsing
Thread pool design for concurrency
Connection persistence and graceful shutdown
Compression and header management
Routing and dynamic path parameters
Perfect for anyone learning Rust systems programming or HTTP internals.

🔮 Future Improvements
Implement middleware and static file serving
Add request pipelining and async I/O support (e.g., Tokio)
Extend router for more advanced parameter handling

👨‍💻 Author
Yeabsira Shimelis
Built with ❤️ and Rust 🦀
