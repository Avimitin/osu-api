use std::collections::HashMap;

use super::{OsuMode, UserId};
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

impl<'k, 'u> TryFrom<GetUserRecentProp<'k, 'u>> for HashMap<&'static str, String> {
  type Error = String;

  fn try_from(prop: GetUserRecentProp<'k, 'u>) -> Result<Self, Self::Error> {
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

    Ok(query)
  }
}

#[derive(Deserialize)]
pub struct GetUserRecentResp {
  beatmap_id: String,
  score: String,
  maxcombo: String,
  count50: String,
  count100: String,
  count300: String,
  countmiss: String,
  countkatu: String,
  countgeki: String,
  perfect: String,
  enabled_mods: String,
  user_id: String,
  date: String,
  rank: String,
}
