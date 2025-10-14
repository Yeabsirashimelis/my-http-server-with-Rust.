use anyhow::{Context, Result};
use serde::Deserialize;
use std::io::BufReader;

use crate::{http::request::build_request, router::Router, server::response::send_response};

pub fn handle_connection(stream: &mut std::net::TcpStream, router: &Router) -> Result<()> {
    let mut stream = stream.try_clone().context("getting the stream")?;
    let mut reader = BufReader::new(&stream);

    // let lines_iter = reader.lines();

    let request = build_request(&mut reader)?;
    println!("Parsed Request :- {:#?}", request);

    let res = router.handle(&request);

    send_response(res, &mut stream).context("returning a response")?;

    Ok(())
}
