pub(crate) use quick_xml::de::from_str;

pub mod rfc3339_to_datetime_utc {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let offset_time = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
        Ok(Some(DateTime::from_naive_utc_and_offset(
            offset_time.naive_utc(),
            Utc,
        )))
    }
}
