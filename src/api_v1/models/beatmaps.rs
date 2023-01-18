use std::collections::HashMap;

use thiserror::Error;
use typed_builder::TypedBuilder;

use super::{Mods, OsuMode};

#[derive(Error, Debug)]
pub enum Error {
  #[error("Argument given is invalid: {0}")]
  InvalidArgument(String),
}

type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum UserId {
  Id(u64),
  Username(String),
}

impl From<u64> for UserId {
  fn from(id: u64) -> Self {
    Self::Id(id)
  }
}

impl From<String> for UserId {
  fn from(name: String) -> Self {
    Self::Username(name)
  }
}

// TODO: Add `since`
#[derive(Debug, TypedBuilder)]
#[builder(builder_type_doc = "Builder for creating request to get_beatmaps API,
read https://github.com/ppy/osu-api/wiki#parameters for meaning")]
pub struct GetBeatmapsProps {
  #[builder(default = 0)]
  beatmapset_id: u64,
  #[builder(default = 0)]
  beatmap_id: u64,
  #[builder(setter(transform = |id: impl Into<UserId>| id.into()))]
  user_id: UserId,
  #[builder(default, setter(strip_option))]
  mode: Option<OsuMode>,
  #[builder(setter(strip_bool))]
  include_converted: bool,
  #[builder(default, setter(strip_option))]
  beatmap_hash: Option<String>,
  #[builder(default = 0)]
  limit: u32,
  #[builder(default = Vec::new())]
  mods: Vec<Mods>,
  #[builder(default, setter(strip_option))]
  since: Option<chrono::NaiveDate>,
}

impl GetBeatmapsProps {
  pub(crate) fn try_into_query_param(self, key: &str) -> Result<HashMap<&'static str, String>> {
    let mut query = HashMap::new();

    query.insert("k", key.to_string());

    if self.beatmapset_id == 0 && self.beatmap_id == 0 {
      return Err(Error::InvalidArgument(
        "neither beatmapset id nor beatmap id was given".to_string(),
      ));
    }

    if self.beatmapset_id != 0 {
      query.insert("s", self.beatmapset_id.to_string());
    }

    match self.user_id {
      UserId::Id(id) => {
        query.insert("u", id.to_string());
        query.insert("type", "id".to_string());
      }
      UserId::Username(name) => {
        query.insert("u", name);
        query.insert("type", "string".to_string());
      }
    };

    if let Some(mode) = self.mode {
      query.insert("m", mode.to_string());

      let include_converted = if self.include_converted { "1" } else { "0" };
      match mode {
        OsuMode::Standard => None, // do nothing
        _ => query.insert("a", include_converted.to_string()),
      };
    }

    if let Some(hash) = self.beatmap_hash {
      query.insert("h", hash);
    }

    if self.limit != 0 {
      query.insert("limit", self.limit.to_string());
    }

    if !self.mods.is_empty() {
      let mods = self.mods.into_iter().fold(0_u64, |accum, item| {
        let bit: u64 = item.into();
        accum | bit
      });

      query.insert("mods", mods.to_string());
    }

    if let Some(date) = self.since {
      query.insert("since", date.to_string());
    }

    Ok(query)
  }
}
