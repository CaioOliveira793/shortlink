use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortUrl {
    /// Short url slug and record ID
    pub slug: String,
    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated: OffsetDateTime,
    pub long_url: String,
    pub active: bool,
    #[serde(with = "crate::util::serde::opt_iso8601")]
    pub expires: Option<OffsetDateTime>,
    pub creator_id: Ulid,
}

impl ShortUrl {
    pub fn new(
        slug: String,
        long_url: String,
        expires: Option<OffsetDateTime>,
        creator_id: Ulid,
    ) -> Self {
        let created = OffsetDateTime::now_utc();
        Self {
            slug,
            created,
            updated: created,
            long_url,
            active: true,
            expires,
            creator_id,
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
