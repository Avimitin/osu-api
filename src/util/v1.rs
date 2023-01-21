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

pub struct LatestReplay {
  pub score: GetUserRecentResp,
  pub beatmap: GetBeatmapsResp,
}

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
