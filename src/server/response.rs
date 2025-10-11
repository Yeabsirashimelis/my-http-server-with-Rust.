use crate::{
    http::response::{self, Response},
    HttpCode,
};
use anyhow::{Context, Result};
use std::{io::Write, net::TcpStream};

pub fn send_response(response: Response, stream: &mut TcpStream) -> Result<()> {
    let response_string = response.to_http();
    stream
        .write_all(response_string.as_bytes())
        .context("writting all response data")?;
    stream.flush().context("flushing data")?;

    Result::Ok(())
}
