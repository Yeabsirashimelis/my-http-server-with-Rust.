pub mod handler;
pub mod response;

use crate::{
    http::response::Response, router::Router, server::handler::handle_connection,
    threadpool::ThreadPool, HttpCode,
};
use anyhow::Result;

use std::{net::TcpListener, sync::Arc, thread::sleep, time::Duration};

pub fn run() -> Result<()> {
    let mut router = Router::new();

    let unknown = String::from("unknown");

    // register your routes
    router.get("/", |_, _| Response::new(HttpCode::Ok, "Welcome Home!"));
    router.get("/about", |_, _| {
        sleep(Duration::from_secs(6));
        Response::new(HttpCode::Ok, "About Page")
    });
    router.get("/user/:id", move |_, params| {
        let id = params.get("id").unwrap_or(&unknown);
        Response::new(HttpCode::Ok, format!("User ID: {}", id))
    });

    let listener = TcpListener::bind("127.0.0.1:4221")?;
    println!("Server running on http://127.0.0.1:4221");

    let pool = ThreadPool::new(100);

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
