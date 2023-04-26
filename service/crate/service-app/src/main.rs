use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use config::env;

mod config;
mod signal;

async fn create_surreal_client() -> Surreal<Client> {
    let surreal_addr = format!("{}:{}", env::get().surreal_host, env::get().surreal_port);

    Surreal::new::<Ws>(surreal_addr)
        .await
        .expect("failed to connect to surrealdb")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], env::get().port));
    let app = Router::new()
        .route("/url", post(|| async { "Hello World!" }))
        .route("/health", get(|| async { "Ok" }));

    let db = create_surreal_client().await;

    db.signin(Root {
        username: &env::get().surreal_user,
        password: &env::get().surreal_password,
    })
    .await
    .expect("failed to signin to surrealdb");

    // db.use_ns("shortlink").use_db("shortlink_db").await.expect("failed to connect ");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal::termination())
        .await
        .expect("failed to start the server");
}
