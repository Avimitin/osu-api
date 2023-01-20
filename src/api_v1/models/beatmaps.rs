use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use typed_builder::TypedBuilder;

use super::{de::*, ModsFlag, OsuMode, UserId};

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

#[derive(Debug)]
pub enum Approval {
  Graveyard = -2,
  WIP = -1,
  Pending = 0,
  Ranked = 1,
  Approved = 2,
  Qualified = 3,
  Loved = 4,
}

/// TODO: convert all type
#[derive(Deserialize)]
pub struct GetBeatmapsResp {
  // 4 = loved, 3 = qualified, 2 = approved, 1 = ranked, 0 = pending, -1 = WIP, -2 = graveyard
  #[serde(deserialize_with = "s_to_approval")]
  pub approved: Approval,
  // date submitted, in UTC
  #[serde(deserialize_with = "s_to_datetime")]
  pub submit_date: DateTime<Utc>,
  // date ranked, in UTC
  #[serde(deserialize_with = "s_to_datetime")]
  pub approved_date: DateTime<Utc>,
  // last update date, in UTC. May be after approved_date if map was unranked and reranked.
  #[serde(deserialize_with = "s_to_datetime")]
  pub last_update: DateTime<Utc>,
  pub artist: String,
  // beatmap_id is per difficulty
  #[serde(deserialize_with = "s_to_u64")]
  pub beatmap_id: u64,
  // beatmapset_id groups difficulties into a set
  #[serde(deserialize_with = "s_to_u64")]
  pub beatmapset_id: u64,
  #[serde(deserialize_with = "s_to_u16")]
  pub bpm: u16,
  pub creator: String,
  #[serde(deserialize_with = "s_to_u64")]
  pub creator_id: u64,
  // The number of stars the map would have in-game and on the website
  #[serde(deserialize_with = "s_to_f64")]
  pub difficultyrating: f64,
  #[serde(deserialize_with = "s_to_f64")]
  pub diff_aim: f64,
  #[serde(deserialize_with = "s_to_f64")]
  pub diff_speed: f64,
  // Circle size value (CS)
  #[serde(deserialize_with = "s_to_u8")]
  pub diff_size: u8,
  // Overall difficulty (OD)
  #[serde(deserialize_with = "s_to_u8")]
  pub diff_overall: u8,
  // Approach Rate (AR)
  #[serde(deserialize_with = "s_to_u8")]
  pub diff_approach: u8,
  // Health drain (HP)
  #[serde(deserialize_with = "s_to_u8")]
  pub diff_drain: u8,
  // seconds from first note to last note not including breaks
  #[serde(deserialize_with = "s_to_u64")]
  pub hit_length: u64,
  pub source: String,
  pub genre_id: String,
  // 0 = any, 1 = unspecified, 2 = english, 3 = japanese, 4 = chinese, 5 = instrumental, 6 = korean, 7 = french, 8 = german, 9 = swedish, 10 = spanish, 11 = italian, 12 = russian, 13 = polish, 14 = other
  pub language_id: String,
  // song name
  pub title: String,
  // seconds from first note to last note including breaks
  pub total_length: String,
  // difficulty name
  pub version: String,
  pub file_md5: String,
  // md5 hash of the beatmap
  // game mode,
  pub mode: String,
  // Beatmap tags separated by spaces.
  pub tags: String,
  // Number of times the beatmap was favourited. (Americans: notice the ou!)
  pub favourite_count: String,
  pub rating: String,
  // Number of times the beatmap was played
  pub playcount: String,
  // Number of times the beatmap was passed, completed (the user didn't fail or retry)
  pub passcount: String,
  pub count_normal: String,
  pub count_slider: String,
  pub count_spinner: String,
  // The maximum combo a user can reach playing this beatmap.
  pub max_combo: String,
  // If this beatmap has a storyboard
  #[serde(deserialize_with = "s_to_bool")]
  pub storyboard: bool,
  // If this beatmap has a video
  #[serde(deserialize_with = "s_to_bool")]
  pub video: bool,
  // If the download for this beatmap is unavailable (old map, etc.)
  #[serde(deserialize_with = "s_to_bool")]
  pub download_unavailable: bool,
  // If the audio for this beatmap is unavailable (DMCA takedown, etc.)
  #[serde(deserialize_with = "s_to_bool")]
  pub audio_unavailable: bool,
}
