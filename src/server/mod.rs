pub mod handler;
pub mod response;

use crate::server::handler::handle_connection;
use anyhow::Result;
use std::net::TcpListener;

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;
    println!("Server running on http://127.0.0.1:4221");

    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_connection(&mut stream)?;
    }

    Ok(())
}
