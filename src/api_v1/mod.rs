mod models;
use std::collections::HashMap;

use paste::paste;

#[async_trait::async_trait]
pub trait OsuApiRequester {
  async fn get_user_recent<'k, 'u>(
    &self,
    param: models::GetUserRecentProp<'k, 'u>,
  ) -> Result<Vec<models::GetUserRecentResp>, Error>;

  async fn get_beatmaps<'u, 'k>(
    &self,
    param: models::GetBeatmapsProps<'u, 'k>,
  ) -> Result<Vec<models::GetBeatmapsResp>, Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("the given request param is invalid")]
  InvalidRequestParams,
  #[error("fail to send request")]
  NetIO(#[from] reqwest::Error),
  #[error("fail to deserialize response into expected type")]
  UnexpectedResponse(#[from] serde_json::Error),
}

/// Generate constant endpoint
macro_rules! generate_endpoint {
  ($($ep:ident;)+) => {
    paste!{
       $(
          const [<API_ $ep:upper>]: &'static str = concat!("https://osu.ppy.sh/api/", stringify!($ep));
       )+
    }
  };
}

generate_endpoint! {
  get_beatmaps;
  get_user;
  get_scores;
  get_user_best;
  get_user_recent;
}

#[async_trait::async_trait]
impl OsuApiRequester for reqwest::Client {
  async fn get_user_recent<'k, 'u>(
    &self,
    param: models::GetUserRecentProp<'k, 'u>,
  ) -> Result<Vec<models::GetUserRecentResp>, Error> {
    let param: HashMap<&'static str, String> = param.into();
    let url = reqwest::Url::parse_with_params(API_GET_USER_RECENT, &param)
      .expect("fail to turn GetUserRecentProp into params");
    let resp = self.get(url).send().await?.bytes().await?;
    let recent: Vec<models::GetUserRecentResp> = serde_json::from_slice(&resp)?;

    Ok(recent)
  }

  async fn get_beatmaps<'u, 'k>(
    &self,
    param: models::GetBeatmapsProps<'u, 'k>,
  ) -> Result<Vec<models::GetBeatmapsResp>, Error> {
    let param: HashMap<&'static str, String> =
      param.try_into().map_err(|_| Error::InvalidRequestParams)?;
    let url = reqwest::Url::parse_with_params(API_GET_BEATMAPS, &param)
      .expect("fail to turn given param into query");
    let resp = self.get(url).send().await?.bytes().await?;
    let recent: Vec<models::GetBeatmapsResp> = serde_json::from_slice(&resp)?;
    Ok(recent)
  }
}

#[tokio::test]
async fn test_get_user_recent() {
  dotenvy::dotenv().ok();

  let api_key = std::env::var("OSU_API_KEY").expect("Require env `OSU_API_KEY` set");

  let props = models::GetUserRecentProp::builder()
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
  let props = models::GetBeatmapsProps::builder()
    .api_key(&api_key)
    .beatmapset_id(896080)
    .build();
  let resp = client.get_beatmaps(props).await.unwrap();
  // tsukinami have 3 difficulties
  assert_eq!(resp.len(), 3);

  // #2 Test get single difficulty
  let props = models::GetBeatmapsProps::builder()
    .api_key(&api_key)
    .beatmap_id(1872396)
    .build();
  let resp = client.get_beatmaps(props).await.unwrap();
  assert_eq!(resp.len(), 1);

  // #3 Test mods
  let props = models::GetBeatmapsProps::builder()
    .api_key(&api_key)
    .beatmap_id(1872396)
    .mods(vec![models::ModsFlag::DOUBLETIME, models::ModsFlag::EASY])
    .build();
  let resp = client.get_beatmaps(props).await.unwrap();
  assert!(!resp.is_empty());

  // ensure mods is applied
  let map = &resp[0];
  assert!(map.difficultyrating > 6.01);
}
