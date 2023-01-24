use super::{
  de::{s_to_bool, s_to_datetime, s_to_mods_flags, s_to_u32, s_to_u64},
  GameMode, ModsFlag, UserId,
};
use crate::api_v1::{req::Query, Error as ReqError};
use serde::Deserialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug)]
pub struct GetUserRecentProp<'k, 'u> {
  api_key: &'k str,
  #[builder(setter(transform = |id: impl Into<UserId<'u>>| id.into()))]
  user_info: UserId<'u>,
  #[builder(default, setter(strip_option))]
  mode: Option<GameMode>,
  #[builder(default = 10)]
  limit: u8,
}

impl<'k, 'u> TryFrom<GetUserRecentProp<'k, 'u>> for Query {
  type Error = ReqError;

  fn try_from(prop: GetUserRecentProp<'k, 'u>) -> std::result::Result<Self, Self::Error> {
    let mut query = Self::new();

    query.push("k", prop.api_key);

    match prop.user_info {
      UserId::Id(number) => {
        query.push("u", number);
      }
      UserId::Username(name) => {
        // Osu! API server will recognize name consist with only number as user ID,
        // so we must explicit set `type` to `string` here to avoid misunderstanding.
        query.push("type", "string");
        query.push("u", name);
      }
    };

    if let Some(mode) = prop.mode {
      query.push("m", mode);
    }

    query.push("limit", prop.limit);

    Ok(query)
  }
}

#[derive(Deserialize, Debug)]
pub struct GetUserRecentResp {
  pub rank: String,
  pub user_id: String,
  #[serde(deserialize_with = "s_to_u64")]
  pub beatmap_id: u64,
  #[serde(deserialize_with = "s_to_u64")]
  pub score: u64,
  #[serde(deserialize_with = "s_to_u32")]
  pub maxcombo: u32,
  #[serde(deserialize_with = "s_to_u32")]
  pub count50: u32,
  #[serde(deserialize_with = "s_to_u32")]
  pub count100: u32,
  #[serde(deserialize_with = "s_to_u32")]
  pub count300: u32,
  #[serde(deserialize_with = "s_to_u32")]
  pub countmiss: u32,
  #[serde(deserialize_with = "s_to_u32")]
  pub countkatu: u32,
  #[serde(deserialize_with = "s_to_u32")]
  pub countgeki: u32,
  #[serde(deserialize_with = "s_to_bool")]
  pub perfect: bool,
  #[serde(deserialize_with = "s_to_mods_flags")]
  pub enabled_mods: ModsFlag,
  #[serde(deserialize_with = "s_to_datetime")]
  pub date: chrono::DateTime<chrono::Utc>,
}
