use std::fmt::Display;

#[derive(Debug)]
pub enum HttpCode {
    Ok,
    NotFound,
    InternalServerError,
    Created,
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (code, message) = match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "Not Found"),
            Self::InternalServerError => (500, "Internal Server Error"),
            Self::Created => (201, "Created"),
        };
        write!(f, "{} {}", code, message)
    }
}
