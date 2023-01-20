use std::collections::HashMap;

use typed_builder::TypedBuilder;

use super::{ModsFlag, OsuMode, UserId};

#[derive(Debug, TypedBuilder)]
#[builder(builder_type_doc = "Builder for creating request to get_beatmaps API,
read https://github.com/ppy/osu-api/wiki#parameters for meaning")]
pub struct GetBeatmapsProps<'u, 'k> {
  api_key: &'k str,
  #[builder(default = 0)]
  beatmapset_id: u64,
  #[builder(default = 0)]
  beatmap_id: u64,
  #[builder(setter(transform = |id: impl Into<UserId<'u>>| id.into()))]
  user_id: UserId<'u>,
  #[builder(default, setter(strip_option))]
  mode: Option<OsuMode>,
  #[builder(setter(strip_bool))]
  include_converted: bool,
  #[builder(default, setter(strip_option))]
  beatmap_hash: Option<String>,
  #[builder(default = 0)]
  limit: u32,
  #[builder(default = Vec::new())]
  mods: Vec<ModsFlag>,
  #[builder(default, setter(strip_option))]
  since: Option<chrono::NaiveDate>,
}

impl<'u, 'k> TryFrom<GetBeatmapsProps<'u, 'k>> for HashMap<&'static str, String> {
  /// There is only one error represent that the beatmapset_id and beatmap_id are both not given
  type Error = ();

  fn try_from(value: GetBeatmapsProps<'u, 'k>) -> std::result::Result<Self, Self::Error> {
    let mut query = HashMap::new();

    query.insert("k", value.api_key.to_string());

    if value.beatmapset_id == 0 && value.beatmap_id == 0 {
      return Err(());
    }

    if value.beatmapset_id != 0 {
      query.insert("s", value.beatmapset_id.to_string());
    }

    match value.user_id {
      UserId::Id(id) => {
        query.insert("u", id.to_string());
      }
      UserId::Username(name) => {
        query.insert("u", name.to_string());
        query.insert("type", "string".to_string());
      }
    };

    if let Some(mode) = value.mode {
      query.insert("m", mode.to_string());

      let include_converted = if value.include_converted { "1" } else { "0" };
      match mode {
        OsuMode::Standard => None, // do nothing
        _ => query.insert("a", include_converted.to_string()),
      };
    }

    if let Some(hash) = value.beatmap_hash {
      query.insert("h", hash);
    }

    if value.limit != 0 {
      query.insert("limit", value.limit.to_string());
    }

    if !value.mods.is_empty() {
      let mods = value
        .mods
        .into_iter()
        .fold(0_u64, |accum, item| accum | item.bits());

      query.insert("mods", mods.to_string());
    }

    if let Some(date) = value.since {
      query.insert("since", date.to_string());
    }

    Ok(query)
  }
}
