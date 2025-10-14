use std::collections::HashMap;

use crate::HttpCode;

#[derive(Debug)]
pub struct Response {
    pub code: HttpCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(code: HttpCode, body: impl Into<Vec<u8>>) -> Self {
        let body_bytes: Vec<u8> = body.into();
        let content_length = body_bytes.len();
        Self {
            code,
            headers: HashMap::from([
                ("backend-server".into(), "Yeabsira.RUST".into()),
                (
                    "Content-Length".into(),
                    format!("{}", content_length).into(),
                ),
            ]),
            body: body_bytes,
        }
    }

    pub fn to_http(&self) -> Vec<u8> {
        let header_str = format!(
            "HTTP/1.1 {}\r\n{}\r\n\r\n",
            self.code,
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join("\r\n"),
        );

        //convert headers to bytes
        let mut response_bytes = header_str.into_bytes();

        // Append body bytes
        response_bytes.extend_from_slice(&self.body);

        response_bytes
    }
}
