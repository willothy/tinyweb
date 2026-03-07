use tinyweb::{Router, server};
use tinyweb_tokio::{TcpIncoming, TokioRuntime};

async fn hello() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().get("/", hello);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:8080");
    server::serve(TcpIncoming(listener), router, TokioRuntime)
        .await
        .unwrap();
}
