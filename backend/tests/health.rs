async fn spawn_app() {
    let server = backend::run("127.0.0.1:0").await.expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[actix_rt::test]
async fn health_works() {
    spawn_app().await;
}
