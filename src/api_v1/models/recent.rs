use std::collections::HashMap;

use super::{
  s_to_bool, s_to_datetime, s_to_mods_flags, s_to_u32, s_to_u64, ModsFlag, OsuMode, UserId,
};
use serde::Deserialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug)]
pub struct GetUserRecentProp<'k, 'u> {
  api_key: &'k str,
  #[builder(setter(transform = |id: impl Into<UserId<'u>>| id.into()))]
  user_info: UserId<'u>,
  #[builder(default, setter(strip_option))]
  mode: Option<OsuMode>,
  #[builder(default = 10)]
  limit: u8,
}

impl<'k, 'u> From<GetUserRecentProp<'k, 'u>> for HashMap<&'static str, String> {
  fn from(prop: GetUserRecentProp<'k, 'u>) -> Self {
    let mut query = Self::new();

    query.insert("k", prop.api_key.to_string());

    match prop.user_info {
      UserId::Id(number) => {
        query.insert("u", number.to_string());
      }
      UserId::Username(name) => {
        // Osu! API server will recognize name consist with only number as user ID,
        // so we must explicit set `type` to `string` here to avoid misunderstanding.
        query.insert("type", "string".to_string());
        query.insert("u", name.to_string());
      }
    };

    if let Some(mode) = prop.mode {
      query.insert("m", mode.to_string());
    }

    query.insert("limit", prop.limit.to_string());

    query
  }
}

#[derive(Deserialize)]
pub struct GetUserRecentResp {
  rank: String,
  user_id: String,
  #[serde(deserialize_with = "s_to_u64")]
  beatmap_id: u64,
  #[serde(deserialize_with = "s_to_u64")]
  score: u64,
  #[serde(deserialize_with = "s_to_u32")]
  maxcombo: u32,
  #[serde(deserialize_with = "s_to_u32")]
  count50: u32,
  #[serde(deserialize_with = "s_to_u32")]
  count100: u32,
  #[serde(deserialize_with = "s_to_u32")]
  count300: u32,
  #[serde(deserialize_with = "s_to_u32")]
  countmiss: u32,
  #[serde(deserialize_with = "s_to_u32")]
  countkatu: u32,
  #[serde(deserialize_with = "s_to_u32")]
  countgeki: u32,
  #[serde(deserialize_with = "s_to_bool")]
  perfect: bool,
  #[serde(deserialize_with = "s_to_mods_flags")]
  enabled_mods: ModsFlag,
  #[serde(deserialize_with = "s_to_datetime")]
  date: chrono::DateTime<chrono::Utc>,
}
