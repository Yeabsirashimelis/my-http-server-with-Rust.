use std::{io::Write, net::TcpListener};

use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream?;

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream
            .write_all(response.as_bytes())
            .context("Writting all response data")?; // write_all ensures that all bytes are written until there is no more data to be written
        stream
            .flush()
            .context("flushing write so that everything goes out")?; // extra check - Flushes this output stream, ensuring that all intermediately buffered contents reach their destination.

        /*
        // using match
         match stream {
            Result::Ok(mut stream) => {
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream
                    .write_all(response.as_bytes())
                    .context("Writting all response data")?; // write_all ensures that all bytes are written until there is no more data to be written
                stream
                    .flush()
                    .context("flushing write so that everything goes out")?; // extra check - Flushes this output stream, ensuring that all intermediately buffered contents reach their destination.
            }
            Result::Err(error) => {
                println!("error: {}", error);
            }
        }
         */
    }

    Result::Ok(())
}
