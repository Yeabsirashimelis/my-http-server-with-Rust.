use crate::HttpCode;

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
            headers: vec![(String::from("name"), String::from("my-http-server"))],
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
                .collect::<Vec<String>>()
                .join("\r\n"),
            self.body
        )
    }
}
