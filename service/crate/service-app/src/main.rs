use std::net::SocketAddr;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));

    let app = Router::new().route("/url", post(|| async { "Hello World!" }));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await
        .expect("failed to start the server");
}
