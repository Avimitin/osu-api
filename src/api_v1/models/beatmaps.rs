use chrono::{DateTime, Utc};
use serde::Deserialize;
use typed_builder::TypedBuilder;

use super::{de::*, GameMode, ModsFlag, UserId};
use crate::api_v1::{req::Query, Error as ReqError};

#[derive(Debug, TypedBuilder)]
#[builder(builder_type_doc = "Builder for creating request to get_beatmaps API,
read https://github.com/ppy/osu-api/wiki#parameters for meaning")]
pub struct GetBeatmapsProps<'u, 'k> {
  api_key: &'k str,
  #[builder(default = 0)]
  beatmapset_id: u64,
  #[builder(default = 0)]
  beatmap_id: u64,
  #[builder(default, setter(transform = |id: impl Into<UserId<'u>>| Some(id.into())))]
  user_id: Option<UserId<'u>>,
  #[builder(default, setter(strip_option))]
  mode: Option<GameMode>,
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

impl<'u, 'k> TryFrom<GetBeatmapsProps<'u, 'k>> for Query {
  type Error = ReqError;

  fn try_from(value: GetBeatmapsProps<'u, 'k>) -> std::result::Result<Self, Self::Error> {
    let mut query = Query::new();

    query.push("k", value.api_key);

    if value.beatmapset_id == 0 && value.beatmap_id == 0 {
      return Err(ReqError::InvalidRequestParams);
    }

    if value.beatmapset_id != 0 {
      query.push("s", value.beatmapset_id);
    }

    if value.beatmap_id != 0 {
      query.push("b", value.beatmap_id);
    }

    if let Some(user_id) = value.user_id {
      match user_id {
        UserId::Id(id) => {
          query.push("u", id);
        }
        UserId::Username(name) => {
          query.push("u", name);
          query.push("type", "string");
        }
      };
    }

    if let Some(mode) = value.mode {
      query.push("m", &mode);

      let include_converted = if value.include_converted { "1" } else { "0" };
      match mode {
        GameMode::Standard => (), // do nothing
        _ => query.push("a", include_converted),
      };
    }

    if let Some(hash) = value.beatmap_hash {
      query.push("h", hash);
    }

    if value.limit != 0 {
      query.push("limit", value.limit);
    }

    if !value.mods.is_empty() {
      let mods = value
        .mods
        .into_iter()
        .fold(0_u64, |accum, item| accum | item.bits());

      query.push("mods", mods);
    }

    if let Some(date) = value.since {
      query.push("since", date);
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
  #[serde(deserialize_with = "s_to_f32")]
  pub diff_size: f32,
  // Overall difficulty (OD)
  #[serde(deserialize_with = "s_to_f32")]
  pub diff_overall: f32,
  // Approach Rate (AR)
  #[serde(deserialize_with = "s_to_f32")]
  pub diff_approach: f32,
  // Health drain (HP)
  #[serde(deserialize_with = "s_to_f32")]
  pub diff_drain: f32,
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
