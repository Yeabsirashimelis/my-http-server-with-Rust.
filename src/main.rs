use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Result::Ok(stream) => {
                println!("accepted new connection");
            }
            Result::Err(error) => {
                println!("error: {}", error);
            }
        }
    }
}
