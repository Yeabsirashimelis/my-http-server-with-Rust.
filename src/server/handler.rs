use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};

use crate::http::{parse_raw_request, HttpCode};

pub fn handle_connection(stream: &mut std::net::TcpStream) -> Result<()> {
    let mut stream = stream.try_clone().context("getting the stream")?;
    let reader = BufReader::new(&stream);
    let request_line = reader.lines().next().transpose()?.unwrap_or_default();

    println!("REQUEST_LINE : {}", request_line);

    let request = parse_raw_request(&request_line)?;
    println!("Parsed Request :- {:#?}", request);

    let response_code = if request.path == "/" {
        HttpCode::Ok
    } else {
        HttpCode::NotFound
    };

    let response = format!("HTTP/1.1 {}\r\n\r\n", response_code);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
