use anyhow::{Context, Result};
use flate2::{write::GzEncoder, Compression};

use std::io::{BufReader, Write};

use crate::{
    http::{
        request::{build_request, Request},
        response::Response,
    },
    router::Router,
    server::response::send_response,
};

fn apply_content_encoding(mut res: Response, req: &Request) -> Response {
    // Check if client accepts gzip
    if Request::accepts_gzip(&req.headers) {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(&res.body).is_ok() {
            if let Ok(compressed) = encoder.finish() {
                println!("COMPRESSED : {:?}", compressed);
                let new_content_length = compressed.len().to_string(); //new Content-Length

                res.body = compressed;
                res.headers.insert("Content-Encoding".into(), "gzip".into());

                //IMPORTANT: update the Content-Length header with the size of the compresses body
                res.headers
                    .insert("Content-Length".into(), new_content_length);
            }
        }
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
