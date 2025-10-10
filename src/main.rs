use std::process;

use http_server::run;

fn main() {
    match run() {
        Result::Ok(_) => println!("server exited"),
        Result::Err(error) => {
            eprintln!("Error; {}", error);
            process::exit(1); // not zero - means exited with error
        }
    }
}
