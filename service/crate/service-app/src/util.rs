pub mod serde {
    use serde::{de::{self, Deserialize, Deserializer}, ser, Serializer};
    use time::OffsetDateTime;

    #[allow(dead_code)]
    pub mod opt_timestamp {
        use super::*;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let ts_opt: Option<i64> = Deserialize::deserialize(deserializer)?;
            if let Some(ts) = ts_opt {
                return OffsetDateTime::from_unix_timestamp(ts)
                    .map_err(de::Error::custom)
                    .map(|v| Some(v));
            }
            Ok(None)
        }

        pub fn serialize<'de, S>(
            time: &Option<OffsetDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match time {
                Some(value) => serializer.serialize_i64(value.unix_timestamp()),
                None => serializer.serialize_none(),
            }
        }
    }

    pub mod opt_iso8601 {
        use time::format_description::well_known::Iso8601;

        use super::*;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let ts_opt: Option<String> = Deserialize::deserialize(deserializer)?;
            if let Some(ts) = ts_opt {
                return OffsetDateTime::parse(&ts, &Iso8601::DEFAULT)
                    .map_err(de::Error::custom)
                    .map(|v| Some(v));
            }
            Ok(None)
        }

        pub fn serialize<'de, S>(
            time: &Option<OffsetDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *time {
                Some(ref value) => {
                    match value.format(&Iso8601::DEFAULT) {
                        Err(_) => {
                            Err(ser::Error::custom("invalid ISO8601 formated date time"))
                        }
                        Ok(s) => serializer.serialize_str(&s),
                    }
                }
                None => serializer.serialize_none(),
            }
        }
    }
}
