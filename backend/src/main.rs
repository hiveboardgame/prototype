use std::net::TcpListener;

use backend::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Valid address");
    run(listener).await?.await
}
