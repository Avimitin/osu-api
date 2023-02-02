mod user;
mod beatmap;
mod beatmapset;
mod covers;

use chrono::DateTime;
use serde::{Deserialize, Deserializer};


pub use user::{User, UserSaver};
pub use beatmap::BeatMap;
pub use beatmap::Failtimes;
pub use beatmapset::BeatMapSet;
pub use covers::Covers;


pub fn s_to_datetime<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
{
    let p = String::deserialize(deserializer);
    if p.is_err() {
        return Ok(None);
    }
    let p = p.unwrap();
    let d = DateTime::parse_from_rfc3339(&p);

    if let Ok(data) = d {
        return Ok(Some(data.timestamp_millis()));
    } else {
        return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&p), &"time format from ISO 8601"));
    }
}

// struct pub 正则替换 (?<=\s)(?<!pub\s+)(?=(\w+):(?!:)\s+\w+,\s*$)