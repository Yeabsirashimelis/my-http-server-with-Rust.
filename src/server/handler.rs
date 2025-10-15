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
    let mut reader = BufReader::new(stream);

    //use loop for persistence connections - use one thread (TCP-stream) for many incoming requests unless the req-header says so
    loop {
        let request = build_request(&mut reader)?;
        println!("Parsed Request :- {:#?}", request);

        let route_res: Response = router.handle(&request);
        let mut res = apply_content_encoding(route_res, &request);
        println!("FINAL RESPONSE: {:#?}", res);

        //default to "keep-alive"
        let connection_header = request
            .headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case("Connection"))
            .map(|(_, value)| value.to_lowercase())
            .unwrap_or_else(|| "keep-alive".to_string());

        if connection_header == "close" {
            res.headers
                .insert("Connection".to_string(), "close".to_string());
            send_response(res, &mut reader.get_mut()).context("returning a response")?;
            break;
        } else {
            res.headers
                .insert("Connection".to_string(), "keep-alive".to_string());
            // reader.get_mut() gives you back the underlying TcpStream for writing.
            send_response(res, &mut reader.get_mut()).context("returning a response")?;
        }

        // send_response(res, &mut stream).context("returning a response")?;
    }
    Ok(())
}
