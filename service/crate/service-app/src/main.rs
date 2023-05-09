use std::net::SocketAddr;

use config::env;

mod config;
mod entity;
mod router;
mod signal;
mod util;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], env::get().port));
    let app = router::make_app_router().await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal::termination())
        .await
        .expect("failed to start the server");
}
