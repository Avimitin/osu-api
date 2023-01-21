use crate::api_v1::{
  Error as ApiError, GetBeatmapsProps, GetBeatmapsResp, GetUserRecentProp, GetUserRecentResp,
  OsuApiRequester, UserId,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("there is no record match your given param")]
  NotFound,
  #[error("internal error")]
  Api(#[from] ApiError),
}

/// Represent the score and beatmap information
pub struct LatestReplay {
  pub score: GetUserRecentResp,
  pub beatmap: GetBeatmapsResp,
}

/// Get the latest replay from the given user
pub async fn get_user_latest_replay<'u, C, U>(
  client: &C,
  key: &str,
  user: U,
) -> Result<LatestReplay, Error>
where
  C: OsuApiRequester,
  U: Into<UserId<'u>>,
{
  let prop = GetUserRecentProp::builder()
    .api_key(key)
    .user_info(user)
    .limit(1)
    .build();

  let mut resp = client.get_user_recent(prop).await?;

  if resp.is_empty() {
    return Err(Error::NotFound);
  }

  let user_recent = resp.swap_remove(0);

  let prop = GetBeatmapsProps::builder()
    .api_key(key)
    .beatmap_id(user_recent.beatmap_id)
    .limit(1)
    .build();

  let mut resp = client.get_beatmaps(prop).await?;

  if resp.is_empty() {
    return Err(Error::NotFound);
  }

  let map_info = resp.swap_remove(0);

  Ok(LatestReplay {
    score: user_recent,
    beatmap: map_info,
  })
}

/// Generate beatmap cover image URL. Require beatmapset_id not beatmap_id.
pub fn gen_beatmap_cover_img_url(set_id: u64) -> reqwest::Url {
  reqwest::Url::parse(&format!(
    "https://assets.ppy.sh/beatmaps/{set_id}/covers/cover.jpg"
  ))
  .unwrap()
}

/// Generate beatmap thumbnail image URL. Require beatmapset_id not beaetmap_id.
pub fn gen_beatmap_thumbnail(set_id: u64) -> reqwest::Url {
  reqwest::Url::parse(&format!("https://b.ppy.sh/thumb/{set_id}l.jpg")).unwrap()
}
