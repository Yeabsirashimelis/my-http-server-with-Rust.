use super::method::Method;
use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    io::{BufReader, Lines},
    net::TcpStream,
    vec,
};

#[derive(Debug)]
pub struct ParsedRequestLine {
    pub method: Method,
    pub path: String,
    pub query_strings: HashMap<String, String>,
}
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query_strings: HashMap<String, String>,
    pub headers: Vec<(String, String)>,
}

pub fn parse_request_line(request_line: &str) -> Result<ParsedRequestLine> {
    let mut parts = request_line.split_whitespace();
    let method_str = parts.next().unwrap_or("GET");
    let path_and_query = parts.next().unwrap_or("/");

    let (path, query_strings) = if let Some((p, q)) = path_and_query.split_once('?') {
        let queries = q
            .split('&')
            .filter_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                Some((k.to_string(), v.to_string()))
            })
            .collect::<HashMap<_, _>>();
        (p.to_string(), queries)
    } else {
        (path_and_query.to_string(), HashMap::new())
    };

    let method = Method::from_str(method_str)?;

    Result::Ok(ParsedRequestLine {
        method,
        path,
        query_strings,
    })
}

pub fn parse_headers(
    mut request_iter: Lines<BufReader<&TcpStream>>,
) -> Result<Vec<(String, String)>> {
    //jump the request line
    let _ = request_iter.next().transpose()?.unwrap_or_default();

    let mut headers: Vec<(String, String)> = vec![];

    for line in request_iter {
        let line = line.unwrap(); // only once per iteration

        if line == "\r\n" || line.is_empty() {
            break; // end of headers
        }

        if let Some((key, value)) = line.split_once(": ") {
            headers.push((key.to_string(), value.to_string()));
        }
    }

    Result::Ok(headers)
}

pub fn build_request(mut request_iter: Lines<BufReader<&TcpStream>>) -> Result<Request> {
    let request_line = request_iter.next().transpose()?.unwrap_or_default();
    println!("REQUEST_LINE : {}", request_line);

    let request_line_parsed = parse_request_line(&request_line)?;
    let parsed_headers = parse_headers(request_iter)?;

    Result::Ok(Request {
        method: request_line_parsed.method,
        path: request_line_parsed.path,
        query_strings: request_line_parsed.query_strings,
        headers: parsed_headers,
    })
}
