use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = backend::run(listener)
        .await
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}

#[actix_rt::test]
async fn health_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{address}/api/health_check"))
        .send()
        .await
        .expect("Failed to execute health_check");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
