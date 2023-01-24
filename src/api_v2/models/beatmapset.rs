use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatMapSet {
    id:u64,
    user_id:u64,
    creator: String,

    artist: String,
    artist_unicode: String,
    title: String,
    title_unicode: String,

    source: String,
    covers:String,
    tags:String,

    preview_url:String,
    legacy_thread_url:String,

    submitted_date: Option<i64>,
    ranked_date: Option<i64>,

    nsfw:bool,
    video:bool,
    can_be_hyped: bool,
    storyboard: bool,
    /// |value|mean|
    /// |---|---|
    /// | -2 | graveyard |
    /// | -1 | wip |
    /// | 0 | pending |
    /// | 1 | ranked |
    /// | 2 | approved |
    /// | 3 | qualified |
    /// | 4 | loved |
    ranked:i8,
    status: String,

    favourite_count:u64,
    play_count:u64,
}
