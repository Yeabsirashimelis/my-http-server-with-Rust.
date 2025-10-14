use anyhow::{Context, Result};

use std::io::BufReader;

use crate::{
    http::{
        request::{build_request, Request},
        response::Response,
    },
    router::Router,
    server::response::send_response,
};

fn apply_content_encoding(mut res: Response, req: &Request) -> Response {
    if let Some(first_encoding) = req.first_accepted_encoding() {
        res.headers
            .insert("Content-Encoding".into(), first_encoding);
    }
    res
}

pub fn handle_connection(stream: &mut std::net::TcpStream, router: &Router) -> Result<()> {
    let mut stream = stream.try_clone().context("getting the stream")?;
    let mut reader = BufReader::new(&stream);

    // let lines_iter = reader.lines();

    let request = build_request(&mut reader)?;
    println!("Parsed Request :- {:#?}", request);

    let route_res: Response = router.handle(&request);
    let res = apply_content_encoding(route_res, &request);
    println!("FINAL RESPONSE: {:#?}", res);

    send_response(res, &mut stream).context("returning a response")?;

    Ok(())
}
