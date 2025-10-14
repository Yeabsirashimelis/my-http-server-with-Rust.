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

    let body_str = std::str::from_utf8(&request.body).context("Request body is not valid UTF-8")?;
    println!("Request Body :- {:#?}", body_str);

    /////////////////////////////////////////////////////////////////////////////

    #[derive(Debug, Deserialize)]
    struct Person {
        name: String,
        work: String,
    }

    let person: Person = serde_json::from_str(body_str)
        .context("Failed to parse inner JSON as the  struct provided")?;
    println!("Request Body :- {:#?}", person);
    //////////////////////////////////////////////////////////////////

    let res = router.handle(&request);

    send_response(res, &mut stream).context("returning a response")?;

    Ok(())
}
