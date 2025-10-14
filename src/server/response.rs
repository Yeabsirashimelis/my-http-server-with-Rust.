use crate::http::response::Response;
use anyhow::{Context, Result};
use std::{io::Write, net::TcpStream};

pub fn send_response(response: Response, stream: &mut TcpStream) -> Result<()> {
    let response_bytes = response.to_http();
    stream
        .write_all(&response_bytes)
        .context("writting all response data")?;
    stream.flush().context("flushing data")?;

    Result::Ok(())
}
