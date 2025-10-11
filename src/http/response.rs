use crate::HttpCode;
use anyhow::{Context, Result};
use std::{io::Write, net::TcpStream};

#[derive(Debug)]
pub struct Response {
    pub code: HttpCode,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn new(code: HttpCode, body: impl Into<String>) -> Self {
        Self {
            code,
            headers: vec![],
            body: body.into(),
        }
    }

    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
            self.code,
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<(String)>>()
                .join("\r\n"),
            self.body
        )
    }
}
