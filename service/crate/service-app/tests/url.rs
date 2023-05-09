use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serial_test::serial;
use service_lib::entity::ShortUrl;
use time::OffsetDateTime;

use setup::setup_test;

mod setup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShortUrlBody {
    long_url: String,
    #[serde(with = "service_lib::util::serde::opt_iso8601")]
    expires: Option<OffsetDateTime>,
    slug: Option<String>,
}

#[tokio::test]
#[serial]
async fn create_url() {
    let (client, url, _) = setup_test().await;

    let dto = CreateShortUrlBody {
        long_url: "http://example.com".into(),
        expires: None,
        slug: None,
    };

    let req = client
        .post(url.join("/url").unwrap())
        .json(&dto)
        .build()
        .unwrap();

    let res = client.execute(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::CREATED,);

    let user: ShortUrl = res.json().await.unwrap();

    assert_eq!(user.long_url, dto.long_url);
    assert_eq!(user.expires, dto.expires);
    assert_eq!(user.slug.len(), 8);
    assert_eq!(user.active, true);
}

#[tokio::test]
#[serial]
async fn create_url_with_slug() {
    let (client, url, _) = setup_test().await;

    let dto = CreateShortUrlBody {
        long_url: "http://example.com".into(),
        expires: None,
        slug: Some("My_Custom_Slug".into()),
    };

    let req = client
        .post(url.join("/url").unwrap())
        .json(&dto)
        .build()
        .unwrap();

    let res = client.execute(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::CREATED,);

    let user: ShortUrl = res.json().await.unwrap();

    assert_eq!(user.long_url, dto.long_url);
    assert_eq!(user.expires, dto.expires);
    assert_eq!(user.slug, dto.slug.unwrap());
    assert_eq!(user.active, true);
}
