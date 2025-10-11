use std::{io::Write, net::TcpStream};

use anyhow::{Context, Result};

use crate::{http::request::Request, HttpCode};

pub fn send_response(response_code: HttpCode, stream: &mut TcpStream) -> Result<()> {
    let response = format!("HTTP/1.1 {}\r\n\r\n", response_code);
    stream
        .write_all(response.as_bytes())
        .context("writting all response data")?;
    stream.flush().context("flushing data")?;

    Result::Ok(())
}
