use serde::{Deserialize, Serialize};
use crate::api_v2::models::covers::Covers;
use super::{s_to_datetime};

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatMapSet {
    pub id: u64,
    pub user_id: u64,
    pub creator: String,

    pub artist: String,
    pub artist_unicode: String,
    pub title: String,
    pub title_unicode: String,

    pub source: String,
    pub covers: Covers,
    pub tags: String,

    pub preview_url: String,
    pub legacy_thread_url: String,

    #[serde(default, deserialize_with = "s_to_datetime")]
    pub submitted_date: Option<i64>,
    #[serde(default, deserialize_with = "s_to_datetime")]
    pub ranked_date: Option<i64>,

    pub nsfw: bool,
    pub video: bool,
    pub can_be_hyped: bool,
    pub storyboard: bool,
    /// |value|mean|
    /// |---|---|
    /// | -2 | graveyard |
    /// | -1 | wip |
    /// | 0 | pending |
    /// | 1 | ranked |
    /// | 2 | approved |
    /// | 3 | qualified |
    /// | 4 | loved |
    pub ranked: i8,
    pub status: String,

    pub bpm: i64,

    pub favourite_count: u64,
    pub play_count: u64,
}
