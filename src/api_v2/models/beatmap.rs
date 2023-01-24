use serde::{Deserialize, Serialize};
use super::{s_to_datetime};

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatMap {
    id: u64,
    user_id: u32,
    beatmapset_id: u64,
    /// 难度('version' 属实是有点怪,我偏向于使用 'difficulty'
    version: String,
    mode: String,
    mode_int: u8,

    difficulty_rating: f32,
    /// Overall Difficulty; yes,ppy uses 'accuracy' as 'overall difficulty'
    accuracy: f32,
    /// Approach Rate
    ar: f32,
    /// Circle Size
    cs: f32,
    // HP Drain Rate
    drain: f32,
    bpm: f32,
    max_combo: u32,
    total_length: u32,
    hit_length: u32,

    /// it is from other mode
    convert: bool,
    /// ranked or loved
    is_scoreable: bool,

    passcount: u32,
    playcount: u32,
    /// is useless, maybe
    url: String,
    /// encode by md5
    checksum: String,

    count_sliders: u32,
    count_spinners: u32,
    count_circles: u32,

    /// |value|mean|
    /// |---|---|
    /// | -2 | graveyard |
    /// | -1 | wip |
    /// | 0 | pending |
    /// | 1 | ranked |
    /// | 2 | approved |
    /// | 3 | qualified |
    /// | 4 | loved |
    ranked: i8,
    /// 'ranked' 'loved' 'wip' 'pending' 'graveyard' ...
    status: String,
    #[serde(default, deserialize_with = "s_to_datetime")]
    deleted_at: Option<i64>,
    #[serde(default, deserialize_with = "s_to_datetime")]
    last_updated: Option<i64>,
}

#[test]
fn test_deserialize() {
    let s = r#"
{
  "difficulty_rating": 5.29,
  "count_sliders": 409,
  "mode_int": 0,
  "accuracy": 8.5,
  "convert": false,
  "passcount": 10211,
  "drain": 5,
  "mode": "osu",
  "is_scoreable": true,
  "playcount": 96337,
  "max_combo": 1952,
  "checksum": "d9169ccee084ceb8855e93d026f71136",
  "ranked": 1,
  "id": 2663395,
  "total_length": 336,
  "bpm": 176,
  "beatmapset_id": 1282289,
  "last_updated": "2021-03-18T05:50:47+00:00",
  "count_spinners": 0,
  "version": "Soliloquy of Broken Tears",
  "url": "https://osu.ppy.sh/beatmaps/2663395",
  "count_circles": 881,
  "cs": 4,
  "ar": 9,
  "user_id": 14094058,
  "hit_length": 321,
  "beatmapset": {
    "artist": "mary",
    "artist_unicode": "めありー",
    "covers": {
      "cover": "https://assets.ppy.sh/beatmaps/1282289/covers/cover.jpg?1622200003",
      "cover@2x": "https://assets.ppy.sh/beatmaps/1282289/covers/cover@2x.jpg?1622200003",
      "card": "https://assets.ppy.sh/beatmaps/1282289/covers/card.jpg?1622200003",
      "card@2x": "https://assets.ppy.sh/beatmaps/1282289/covers/card@2x.jpg?1622200003",
      "list": "https://assets.ppy.sh/beatmaps/1282289/covers/list.jpg?1622200003",
      "list@2x": "https://assets.ppy.sh/beatmaps/1282289/covers/list@2x.jpg?1622200003",
      "slimcover": "https://assets.ppy.sh/beatmaps/1282289/covers/slimcover.jpg?1622200003",
      "slimcover@2x": "https://assets.ppy.sh/beatmaps/1282289/covers/slimcover@2x.jpg?1622200003"
    },
    "creator": "Airiesu",
    "favourite_count": 287,
    "id": 1282289,
    "nsfw": false,
    "play_count": 96337,
    "preview_url": "//b.ppy.sh/preview/1282289.mp3",
    "source": "",
    "status": "ranked",
    "title": "Alice in Reitouko",
    "title_unicode": "Alice in 冷凍庫",
    "user_id": 14094058,
    "video": false,
    "availability": {
      "download_disabled": false
    },
    "bpm": 88,
    "can_be_hyped": false,
    "discussion_enabled": true,
    "discussion_locked": false,
    "is_scoreable": true,
    "last_updated": "2021-03-18T05:50:46+00:00",
    "legacy_thread_url": "https://osu.ppy.sh/community/forums/topics/1165922",
    "nominations_summary": {
      "current": 2,
      "required": 2
    },
    "ranked": 1,
    "ranked_date": "2021-03-21T07:43:05+00:00",
    "storyboard": true,
    "submitted_date": "2020-10-20T14:13:29+00:00",
    "tags": "orangestar mikanseip vocaloid ia めありーちゃんねる meari usagi meausa cover utaite jpop j-pop japanese pop female vocals freezer risru seaside soliloquies",
    "ratings": [
      0,
      2,
      0,
      0,
      0,
      1,
      1,
      0,
      2,
      7,
      121
    ]
  },
  "status": "ranked"
}"#;
    let b = serde_json::from_str::<BeatMap>(s);
    println!("{:?}", b);
}