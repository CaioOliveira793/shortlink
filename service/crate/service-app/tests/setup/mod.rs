use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use service_lib::config::database;
use surrealdb::{engine::remote::ws::Client as SurrealClient, Surreal};
use url::Url;

use std::time::Duration;

const RESET_DB_QUERY: &'static str = "REMOVE TABLE url";

pub async fn setup_test() -> (Client, Url, Surreal<SurrealClient>) {
    dotenv::dotenv().unwrap();
    let db = database::create_surrealdb_client().await;
    db.query(RESET_DB_QUERY).await.unwrap();
    (create_client(), service_url(), db)
}

fn service_url() -> Url {
    let port: u16 = std::env::var("PORT")
        .unwrap()
        .parse()
        .expect("Invalid PORT");
    Url::parse(format!("http://localhost:{port}").as_str()).unwrap()
}

fn create_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.append("accept", HeaderValue::from_static("application/json"));

    let keep_alive = 1000 * 60 * 60; // 1 hours
    let connect_timeout = 1000 * 5; // 5 sec
    let timeout = 1000 * 10; // 10 sec

    reqwest::Client::builder()
        .tcp_keepalive(Duration::from_millis(keep_alive))
        .connect_timeout(Duration::from_millis(connect_timeout))
        .timeout(Duration::from_millis(timeout))
        .pool_max_idle_per_host(5)
        .default_headers(headers)
        .gzip(true)
        .build()
        .expect("Expect to create a http client")
}
