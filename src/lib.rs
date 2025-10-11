use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("starting to listen");

    println!("receiving incoming messages");
    for stream in listener.incoming() {
        let mut stream = stream?;

        let reader = BufReader::new(&stream);
        let request_line = reader.lines().next();

        let request_line_content = match request_line {
            Option::Some(Ok(line)) => line,
            Option::Some(Err(error)) => {
                return Result::Err(error).context("reading request line");
            }
            Option::None => "".to_string(),
        };

        println!("REQUEST_LINE : {}", request_line_content);

        let request = parse_raw_request(&request_line_content).context("Parsing raw request")?;

        println!("Parsed Request :- {:#?}", request);

        let response_code = if request.path == "/" {
            HttpCode::Ok
        } else {
            HttpCode::NotFound
        };

        let response = format!("HTTP/1.1 {}\r\n\r\n", response_code);

        stream
            .write_all(response.as_bytes())
            .context("Writting all response data")?; // write_all ensures that all bytes are written until there is no more data to be written
        stream
            .flush()
            .context("flushing write so that everything goes out")?; // extra check - Flushes this output stream, ensuring that all intermediately buffered contents reach their destination.

        /*
        // using match
         match stream {
            Result::Ok(mut stream) => {
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream
                    .write_all(response.as_bytes())
                    .context("Writting all response data")?; // write_all ensures that all bytes are written until there is no more data to be written
                stream
                    .flush()
                    .context("flushing write so that everything goes out")?; // extra check - Flushes this output stream, ensuring that all intermediately buffered contents reach their destination.
            }
            Result::Err(error) => {
                println!("error: {}", error);
            }
        }
         */
    }

    Result::Ok(())
}

fn parse_raw_request(request_line: &str) -> Result<Request> {
    let mut parts = request_line.split_whitespace();
    let method_str = parts.next().context("Missing HTTP method")?;

    let path_and_query_strings = parts.next().unwrap_or("/");
    let mut path = String::new();
    let mut query_strings: HashMap<String, String> = HashMap::new();
    if path_and_query_strings.contains("?") {
        let temp_string: Vec<&str> = path_and_query_strings.split("?").collect();
        path = temp_string[0].to_string();

        let query_string_str = temp_string[1];
        //                                      yup, lemme use turbofish operator for fun here
        let query_string_arr = query_string_str.split("&").collect::<Vec<&str>>();
        println!("query string: {:#?}", query_string_arr);

        for query in query_string_arr {
            let splitted_query: Vec<_> = query.split("=").collect();
            query_strings.insert(splitted_query[0].to_string(), splitted_query[1].to_string());
        }
    } else {
        path = path_and_query_strings.to_string();
    }
    let method = match method_str {
        "GET" => Method::GET,
        other => return Result::Err(anyhow::anyhow!("Unsupported method: {}", other)),
    };

    Result::Ok(Request {
        method,
        path: path.to_string(),
        query_strings: query_strings,
        protocol: 200,
        headers: HashMap::new(),
        body: "body".to_string(),
    })
}

#[derive(Debug)]
struct Request {
    method: Method,
    path: String,
    query_strings: HashMap<String, String>,
    protocol: i32,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

enum HttpCode {
    Ok,
    NotFound,
}
impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (number, message) = match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "Not Found"),
        };

        write!(f, "{} {}", number, message)
    }
}
