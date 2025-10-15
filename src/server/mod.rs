pub mod handler;
pub mod response;

use crate::{
    http::response::Response, router::Router, server::handler::handle_connection,
    threadpool::ThreadPool, HttpCode,
};
use anyhow::Result;

use std::{fs::File, io::Read, net::TcpListener, sync::Arc};

pub fn run() -> Result<()> {
    let mut router = Router::new();

    let unknown = String::from("unknown");

    // register your routes
    router.get("/", |_, _| Response::new(HttpCode::Ok, "Welcome Home!"));
    router.get("/about", |_, _| {
        let mut response = Response::new(HttpCode::Ok, "<h1>About Page</h1>");
        response
            .headers
            .insert("Content-Type".to_string(), "text/html".to_string());

        response
    });
    router.get("/user/:id", move |_, params| {
        let id = params.get("id").unwrap_or(&unknown);
        Response::new(HttpCode::Ok, format!("User ID: {}", id))
    });

    let directory = std::path::Path::new("files").canonicalize().unwrap();

    router.get("/files/:filepath", move |_, params| {
        let unknown = String::from("unknown");
        // 1. Get the filename from URL
        let filename = params.get("filepath").unwrap_or(&unknown);

        // 2. Build full path relative to "files" directory
        let path = directory.join(filename);

        // 3. Open the file
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return Response::new(HttpCode::NotFound, "file not found"), // 404 if missing
        };

        // 4. Read file into bytes
        let mut buffer = Vec::new();
        if let Err(_) = file.read_to_end(&mut buffer) {
            return Response::new(HttpCode::InternalServerError, "Error reading file");
        }

        // 5. Build response with headers
        let mut resp = Response::new(HttpCode::Ok, buffer);
        resp.headers.insert(
            "Content-Type".to_string(),
            "application/octet-stream".to_string(),
        );

        resp
    });

    let listener = TcpListener::bind("127.0.0.1:4221")?;
    println!("Server running on http://127.0.0.1:4221");

    let pool = ThreadPool::new(500);

    let router = Arc::new(router);

    for stream in listener.incoming() {
        let mut stream = stream?;
        let router = Arc::clone(&router);

        pool.execute(move || {
            if let Err(error) = handle_connection(&mut stream, &router) {
                eprintln!("Error handling connection: {:?}", error);
            }
        });
    }

    Ok(())
}
