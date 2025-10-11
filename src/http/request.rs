use super::method::Method;
use anyhow::{Context, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query_strings: HashMap<String, String>,
}

pub fn parse_raw_request(request_line: &str) -> Result<Request> {
    let mut parts = request_line.split_whitespace();
    let method_str = parts.next().context("Missing HTTP method")?;
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
    Result::Ok(Request {
        method,
        path,
        query_strings,
    })
}
