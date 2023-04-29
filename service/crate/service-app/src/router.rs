use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::config;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Client>,
}

pub async fn make_app_router() -> Router {
    let db = config::create_surrealdb_client().await;
    Router::new()
        .route("/url", get(short_url_service::list_short_url))
        .route("/url", post(short_url_service::create_short_url))
        .route("/health", get(|| async { "Ok" }))
        .route("/:slug", get(short_url_service::redirect_short_url))
        .with_state(AppState { db })
}

pub mod short_url_service {
    use axum::{
        body::Empty,
        extract::{Path, State},
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use serde::{Deserialize, Serialize};
    use sha2::{Digest, Sha256};
    use surrealdb::{engine::remote::ws::Client, Surreal};
    use time::{format_description::well_known::Rfc2822, OffsetDateTime};
    use ulid::Ulid;

    use super::AppState;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ShortUrl {
        /// Short url slug and record ID
        slug: String,
        #[serde(with = "time::serde::iso8601")]
        created: OffsetDateTime,
        #[serde(with = "time::serde::iso8601")]
        updated: OffsetDateTime,
        long_url: String,
        active: bool,
        #[serde(with = "crate::util::serde::opt_iso8601")]
        expires: Option<OffsetDateTime>,
        creator_id: Ulid,
    }

    impl ShortUrl {
        pub fn new(
            slug: String,
            long_url: String,
            expires: Option<OffsetDateTime>,
            creator: Ulid,
        ) -> Self {
            let created = OffsetDateTime::now_utc();
            Self {
                slug,
                created,
                updated: created,
                long_url,
                active: true,
                expires,
                creator_id: creator,
            }
        }

        /// Verify if the ShortUrl is expired
        pub fn expired(&self) -> bool {
            if let Some(expires) = self.expires {
                return expires < OffsetDateTime::now_utc();
            }
            return false;
        }

        /// Verify if the ShortUrl can be redirected
        pub fn responsive(&self) -> bool {
            return self.active && !self.expired();
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ShortUrlRecord {
        #[serde(with = "time::serde::iso8601")]
        created: OffsetDateTime,
        #[serde(with = "time::serde::iso8601")]
        updated: OffsetDateTime,
        long_url: String,
        active: bool,
        #[serde(with = "crate::util::serde::opt_iso8601")]
        expires: Option<OffsetDateTime>,
        creator_id: Ulid,
    }

    impl ShortUrlRecord {
        pub fn to_entity(self, slug: String) -> ShortUrl {
            ShortUrl {
                slug,
                created: self.created,
                updated: self.updated,
                long_url: self.long_url,
                active: self.active,
                expires: self.expires,
                creator_id: self.creator_id,
            }
        }
    }

    impl From<ShortUrl> for ShortUrlRecord {
        fn from(value: ShortUrl) -> Self {
            Self {
                created: value.created,
                updated: value.updated,
                long_url: value.long_url,
                active: value.active,
                expires: value.expires,
                creator_id: value.creator_id,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateShortUrlBody {
        long_url: String,
        #[serde(with = "crate::util::serde::opt_iso8601")]
        expires: Option<OffsetDateTime>,
        slug: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum CreateShortUrlError {
        ShortUrlExist,
    }

    /// Make a short url enconding with base62 the sha256 hash of the
    /// `long_url` with 4 random bytes.
    ///
    /// ```text
    /// short_url = base62( SHA256( long_url + random32 ) )[..8]
    /// ```
    fn make_random_short_url(long_url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(long_url);
        hasher.update(rand::random::<[u8; 4]>());
        let hash = hasher.finalize();
        let mut based = base62::encode(&hash);
        based.truncate(8);
        based
    }

    async fn verify_short_url_exists(slug: &str, db: &Surreal<Client>) -> bool {
        let record: Option<ShortUrlRecord> = db.select(("url", slug)).await.unwrap();
        record.is_some()
    }

    pub async fn create_short_url(
        State(state): State<AppState>,
        Json(data): Json<CreateShortUrlBody>,
    ) -> Result<(StatusCode, Json<ShortUrl>), (StatusCode, Json<CreateShortUrlError>)> {
        let slug = if let Some(slug) = data.slug {
            if verify_short_url_exists(&slug, &state.db).await {
                return Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(CreateShortUrlError::ShortUrlExist),
                ));
            }

            slug
        } else {
            let mut short_url_exists = true;
            let mut short_url = String::new();

            // NOTE: very inefficient for large datasets due to high probability of collisions
            while short_url_exists {
                short_url = make_random_short_url(&data.long_url);
                short_url_exists = verify_short_url_exists(&short_url, &state.db).await;
            }

            short_url
        };

        let entity = ShortUrl::new(slug.clone(), data.long_url, data.expires, Ulid::new());

        let _: ShortUrlRecord = state
            .db
            .create(("url", slug.as_str()))
            .content(ShortUrlRecord::from(entity.clone()))
            .await
            .unwrap();

        Ok((StatusCode::CREATED, Json(entity)))
    }

    pub async fn redirect_short_url(
        Path(slug): Path<String>,
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let res: Option<ShortUrlRecord> = state.db.select(("url", slug.as_str())).await.unwrap();

        if let Some(record) = res {
            let entity = record.to_entity(slug);

            if entity.responsive() {
                let mut response = Response::builder()
                    .status(StatusCode::FOUND)
                    .header("Location", &entity.long_url);

                if let Some(e) = entity.expires {
                    let mut buf = Vec::new();
                    e.format_into(&mut buf, &Rfc2822).unwrap();
                    response = response.header("Expires", buf)
                }

                let response = response.body(Empty::new()).unwrap();
                return Ok(response);
            }
        }

        Err(StatusCode::NOT_FOUND)
    }

    pub async fn list_short_url(
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let slug: String = "fake_slug".into();
        let records: Vec<ShortUrlRecord> = state.db.select("url").await.unwrap();
        let entities = records.into_iter().map(|rec| rec.to_entity(slug.clone())).collect::<Vec<_>>();
        Ok((StatusCode::OK, Json(entities)))
    }
}
