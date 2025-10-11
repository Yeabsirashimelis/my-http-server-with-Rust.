use std::fmt::Display;

#[derive(Debug)]
pub enum HttpCode {
    Ok,
    NotFound,
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (code, message) = match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "Not Found"),
        };
        write!(f, "{} {}", code, message)
    }
}
