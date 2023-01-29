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

#[test]
fn test_deserialize() {
    let s = r#"
{
  "beatmapset_id" : 919187,
  "difficulty_rating" : 6.02,
  "id" : 1919312,
  "mode" : "osu",
  "status" : "ranked",
  "total_length" : 323,
  "user_id" : 5122187,
  "version" : "We are all MILLION!!",
  "accuracy" : 9.3,
  "ar" : 9.2,
  "bpm" : 172,
  "convert" : false,
  "count_circles" : 722,
  "count_sliders" : 501,
  "count_spinners" : 4,
  "cs" : 3.8,
  "deleted_at" : null,
  "drain" : 6,
  "hit_length" : 302,
  "is_scoreable" : true,
  "last_updated" : "2019-07-14T11:14:55Z",
  "mode_int" : 0,
  "passcount" : 439929,
  "playcount" : 6383511,
  "ranked" : 1,
  "url" : "https://osu.ppy.sh/beatmaps/1919312",
  "checksum" : "e3c294d071daf2021ad9945e3f7727dd",
  "beatmapset" : {
    "artist" : "765 MILLION ALLSTARS",
    "artist_unicode" : "765 MILLION ALLSTARS",
    "covers" : {
      "cover" : "https://assets.ppy.sh/beatmaps/919187/covers/cover.jpg?1648720708",
      "cover@2x" : "https://assets.ppy.sh/beatmaps/919187/covers/cover@2x.jpg?1648720708",
      "card" : "https://assets.ppy.sh/beatmaps/919187/covers/card.jpg?1648720708",
      "card@2x" : "https://assets.ppy.sh/beatmaps/919187/covers/card@2x.jpg?1648720708",
      "list" : "https://assets.ppy.sh/beatmaps/919187/covers/list.jpg?1648720708",
      "list@2x" : "https://assets.ppy.sh/beatmaps/919187/covers/list@2x.jpg?1648720708",
      "slimcover" : "https://assets.ppy.sh/beatmaps/919187/covers/slimcover.jpg?1648720708",
      "slimcover@2x" : "https://assets.ppy.sh/beatmaps/919187/covers/slimcover@2x.jpg?1648720708"
    },
    "creator" : "Fu3ya_",
    "favourite_count" : 3422,
    "hype" : null,
    "id" : 919187,
    "nsfw" : false,
    "offset" : 0,
    "play_count" : 6383511,
    "preview_url" : "//b.ppy.sh/preview/919187.mp3",
    "source" : "アイドルマスター ミリオンライブ！",
    "spotlight" : false,
    "status" : "ranked",
    "title" : "UNION!!",
    "title_unicode" : "UNION!!",
    "track_id" : null,
    "user_id" : 5122187,
    "video" : false,
    "availability" : {
      "download_disabled" : false,
      "more_information" : "https://gist.githubusercontent.com/peppy/ac8678e18b66acec8a9529a0cc6fd44a/raw"
    },
    "bpm" : 172,
    "can_be_hyped" : false,
    "discussion_enabled" : true,
    "discussion_locked" : false,
    "is_scoreable" : true,
    "last_updated" : "2019-07-14T11:14:54Z",
    "legacy_thread_url" : "https://osu.ppy.sh/community/forums/topics/863004",
    "nominations_summary" : {
      "current" : 2,
      "required" : 2
    },
    "ranked" : 1,
    "ranked_date" : "2019-07-26T00:00:28Z",
    "storyboard" : false,
    "submitted_date" : "2019-01-31T13:45:02Z",
    "tags" : "アイマス ミリマス ミリシタ idolmaster 堀江晶太 imas im@s 765pro idolm@ster million live the@ter generation 11 mltd japanese idol",
    "ratings" : [ 0, 85, 3, 3, 6, 5, 16, 37, 80, 228, 2832 ]
  },
  "failtimes" : {
    "fail" : [ 0, 0, 1, 17908, 46936, 78399, 192388, 23949, 55328, 81878, 64085, 19187, 20884, 109598, 50483, 60774, 15360, 70098, 198124, 11429, 5397, 4354, 3163, 3241, 3027, 7197, 11120, 13541, 3612, 2809, 2769, 30952, 65776, 8579, 39144, 2032, 279, 12560, 9930, 2105, 2371, 4360, 2882, 1570, 1946, 8281, 5346, 5017, 1458, 25735, 71920, 23925, 14249, 1437, 810, 1236, 608, 804, 2087, 1393, 1004, 610, 828, 11254, 33348, 6454, 5441, 18517, 8398, 13096, 4984, 68820, 89736, 9, 0, 2754, 302, 534, 20053, 3680, 767, 327, 2101, 26481, 18608, 598, 1642, 878, 444, 1, 499, 237, 801, 1037, 428, 747, 2284, 15967, 133562, 17729 ],
    "exit" : [ 0, 0, 54, 451693, 216781, 257771, 361370, 202922, 246227, 180710, 223648, 73017, 92207, 214912, 102264, 82430, 23187, 50477, 113457, 55785, 16964, 24241, 21324, 13182, 24773, 17641, 19508, 41327, 22837, 17071, 6341, 34481, 77537, 21052, 44693, 60574, 34898, 13735, 20934, 14767, 15524, 10979, 16934, 8388, 8255, 12409, 10054, 6637, 2517, 6155, 16812, 8268, 10099, 6405, 3442, 2773, 3522, 2332, 2977, 4524, 4226, 2030, 913, 5641, 11058, 6983, 7392, 6947, 3673, 3243, 1230, 4296, 23894, 8572, 1953, 642, 320, 327, 2714, 3676, 1300, 681, 918, 3415, 4606, 1030, 835, 528, 2267, 3063, 1096, 339, 1157, 1897, 965, 1564, 4909, 11659, 31890, 37380 ]
  },
  "max_combo" : 1812
}"#;
    let b = serde_json::from_str::<BeatMap>(s);
    match b {
        Ok(d) => {
            println!("{:#?}", d);
        }
        Err(d) => {
            println!("error \n{:?}", d);
        }
    };
}