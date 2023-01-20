use std::collections::HashMap;

use crate::api_v1::{
  ApiEndpoint, Error, GetBeatmapsProps, GetBeatmapsResp, GetUserRecentProp, GetUserRecentResp,
  OsuApiRequester,
};

type QueryParam = HashMap<&'static str, String>;

#[async_trait::async_trait]
impl OsuApiRequester for reqwest::Client {
  async fn get_user_recent<'k, 'u>(
    &self,
    param: GetUserRecentProp<'k, 'u>,
  ) -> Result<Vec<GetUserRecentResp>, Error> {
    let param: QueryParam = param.into();
    let url = reqwest::Url::parse_with_params(ApiEndpoint::GET_USER_RECENT, &param)
      .expect("fail to turn GetUserRecentProp into params");
    let resp = self.get(url).send().await?.bytes().await?;
    let recent: Vec<GetUserRecentResp> = serde_json::from_slice(&resp)?;

    Ok(recent)
  }

  async fn get_beatmaps<'u, 'k>(
    &self,
    param: GetBeatmapsProps<'u, 'k>,
  ) -> Result<Vec<GetBeatmapsResp>, Error> {
    let param: HashMap<&'static str, String> =
      param.try_into().map_err(|_| Error::InvalidRequestParams)?;
    let url = reqwest::Url::parse_with_params(ApiEndpoint::GET_BEATMAPS, &param)
      .expect("fail to turn given param into query");
    let resp = self.get(url).send().await?.bytes().await?;
    let recent: Vec<GetBeatmapsResp> = serde_json::from_slice(&resp)?;
    Ok(recent)
  }
}

#[tokio::test]
async fn test_get_user_recent() {
  dotenvy::dotenv().ok();

  let api_key = std::env::var("OSU_API_KEY").expect("Require env `OSU_API_KEY` set");

  let props = GetUserRecentProp::builder()
    .api_key(&api_key)
    .user_info("BlackDog5")
    .limit(1)
    .build();
  let client = reqwest::Client::new();
  let resp = client.get_user_recent(props).await.unwrap();

  assert!(!resp.is_empty())
}

#[tokio::test]
async fn test_get_beatmaps() {
  dotenvy::dotenv().ok();

  let api_key = std::env::var("OSU_API_KEY").expect("Require env `OSU_API_KEY` set");
  let client = reqwest::Client::new();

  // #1 Test get beatmap set
  let props = GetBeatmapsProps::builder()
    .api_key(&api_key)
    .beatmapset_id(896080)
    .build();
  let resp = client.get_beatmaps(props).await.unwrap();
  // tsukinami have 3 difficulties
  assert_eq!(resp.len(), 3);

  // #2 Test get single difficulty
  let props = GetBeatmapsProps::builder()
    .api_key(&api_key)
    .beatmap_id(1872396)
    .build();
  let resp = client.get_beatmaps(props).await.unwrap();
  assert_eq!(resp.len(), 1);

  use super::ModsFlag;

  // #3 Test mods
  let props = GetBeatmapsProps::builder()
    .api_key(&api_key)
    .beatmap_id(1872396)
    .mods(vec![ModsFlag::DOUBLETIME, ModsFlag::EASY])
    .build();
  let resp = client.get_beatmaps(props).await.unwrap();
  assert!(!resp.is_empty());

  // ensure mods is applied
  let map = &resp[0];
  assert!(map.difficultyrating > 6.01);
}
