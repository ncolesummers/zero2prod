use std::fmt::format;
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
use reqwest;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    // No .await call, therefore no need for `spawn_app` to be async now.
    // We are also running tests, so it is not worth it to propagate errors:
    // if we fail to perform the required setup we can just panic and crash
    // all the things.
    fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
        // Retrieve the port assigned to us by the OS
        let port = listener.local_addr().unwrap().port();
        let server = zero2prod::run(listener).expect("Failed to bind address");
        let _ = tokio::spawn(server);
        // We return the application address to the caller
        format!("http://127.0.0.1:{}", port)
    }
}
