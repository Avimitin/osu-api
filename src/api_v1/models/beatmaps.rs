use typed_builder::TypedBuilder;

use super::{Mods, OsuMode};

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
  pub(crate) fn into_query_param(self, key: &str) -> Vec<(String, String)> {
    let mut query: Vec<(String, String)> = Vec::with_capacity(9);

    query.push(("k".to_string(), key.to_string()));

    if self.beatmapset_id == 0 && self.beatmap_id == 0 {
      // TODO: return error here
    }

    if self.beatmapset_id != 0 {
      query.push(("s".to_string(), self.beatmapset_id.to_string()))
    }

    match self.user_id {
      UserId::Id(id) => {
        query.push(("u".to_string(), id.to_string()));
        query.push(("type".to_string(), "id".to_string()));
      }
      UserId::Username(name) => {
        query.push(("u".to_string(), name));
        query.push(("type".to_string(), "string".to_string()));
      }
    };

    if let Some(mode) = self.mode {
      query.push(("m".to_string(), mode.to_string()));

      let include_converted = if self.include_converted { "1" } else { "0" };
      match mode {
        OsuMode::Standard => (), // do nothing
        _ => query.push(("a".to_string(), include_converted.to_string())),
      }
    }

    if let Some(hash) = self.beatmap_hash {
      query.push(("h".to_string(), hash))
    }

    if self.limit != 0 {
      query.push(("limit".to_string(), self.limit.to_string()))
    }

    if !self.mods.is_empty() {
      let mods = self.mods.into_iter().fold(0_u64, |accum, item| {
        let bit: u64 = item.into();
        accum | bit
      });

      query.push(("mods".to_string(), mods.to_string()))
    }

    if let Some(date) = self.since {
      query.push(("since".to_string(), date.to_string()))
    }

    query
  }
}
