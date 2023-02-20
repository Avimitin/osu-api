use serde::{Deserialize, Serialize};
use crate::api_v2::BeatMapSet;
use super::{s_to_datetime};

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatMap {
    pub id: u64,
    pub user_id: u32,
    pub beatmapset_id: u64,
    /// 难度('version' 属实是有点怪,我偏向于使用 'difficulty'
    pub version: String,
    pub mode: String,
    pub mode_int: u8,

    pub difficulty_rating: f32,
    /// Overall Difficulty; yes,ppy uses 'accuracy' as 'overall difficulty'
    pub accuracy: f32,
    /// Approach Rate
    pub ar: f32,
    /// Circle Size
    pub cs: f32,
    // HP Drain Rate
    pub drain: f32,
    pub bpm: f32,
    pub max_combo: u32,
    pub total_length: u32,
    pub hit_length: u32,

    /// it is from other mode
    pub convert: bool,
    /// ranked or loved
    pub is_scoreable: bool,

    pub passcount: u32,
    pub playcount: u32,
    /// is useless, maybe
    pub url: String,
    /// encode by md5
    pub checksum: String,

    pub count_sliders: u32,
    pub count_spinners: u32,
    pub count_circles: u32,

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
    /// 'ranked' 'loved' 'wip' 'pending' 'graveyard' ...
    pub status: String,
    #[serde(default, deserialize_with = "s_to_datetime")]
    pub deleted_at: Option<i64>,
    #[serde(default, deserialize_with = "s_to_datetime")]
    pub last_updated: Option<i64>,

    pub beatmapset: BeatMapSet,
    pub failtimes: Failtimes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Failtimes {
    pub fail: Vec<i64>,
    pub exit: Vec<i64>,
}
