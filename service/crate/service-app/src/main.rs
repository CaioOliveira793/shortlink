use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{
    engine::remote::ws::Ws,
    opt::auth::{Root, Scope},
    Surreal,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3333));
    let app = Router::new()
        .route("/url", post(|| async { "Hello World!" }))
        .route("/health", get(|| async { "Ok" }));

    let db = Surreal::new::<Ws>("surrealdb:2080")
        .await
        .expect("failed to connect to surrealdb");

    db.signin(Scope {
        namespace: "test",
        database: "test",
        scope: "user",
        params: Root {
            password: "root",
            username: "root",
        },
    })
    .await
    .expect("failed to signin to surrealdb");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await
        .expect("failed to start the server");
}
