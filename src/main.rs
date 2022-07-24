use std::net::TcpListener;
use zero2prod::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let _ = run(listener).expect("Failed to bind address");
    Ok(())
}
