use anyhow::Result;
use http_server::run; // thin wrapper, calls the library

fn main() -> Result<()> {
    run()
}
