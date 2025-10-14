use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};

use crate::{
    http::{request::build_request, response::Response, HttpCode},
    router::{self, Router},
    server::response::send_response,
};

fn home_handler(_path: &str) -> Response {
    Response::new(HttpCode::Ok, "Welcome home!")
}

fn user_handler(path: &str) -> Response {
    Response::new(HttpCode::Ok, &format!("User route hit: {}", path))
}

pub fn handle_connection(stream: &mut std::net::TcpStream, router: &Router) -> Result<()> {
    let mut stream = stream.try_clone().context("getting the stream")?;
    let reader = BufReader::new(&stream);

    let mut lines_iter = reader.lines();

    let request = build_request(lines_iter)?;
    println!("Parsed Request :- {:#?}", request);

    let res = router.handle(&request);

    send_response(res, &mut stream).context("returning a response")?;

    Ok(())
}
