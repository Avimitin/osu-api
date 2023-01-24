use std::fmt::Display;

use crate::api_v1::{ApiEndpoint, Error, GetBeatmapsResp, GetUserRecentResp, OsuApiRequester};

pub struct Query {
  pair: Vec<String>,
}

impl Query {
  pub fn new() -> Self {
    Self { pair: Vec::new() }
  }

  pub fn push(&mut self, key: impl Display, val: impl Display) {
    self.pair.push(format!("{key}={val}"))
  }

  pub fn into_query_str(self) -> String {
    self.pair.into_iter().fold(String::new(), |accum, item| {
      if accum.is_empty() {
        item
      } else {
        format!("{accum}&{item}")
      }
    })
  }
}

macro_rules! impl_reqwest {
  (
    $(
      $name:ident {
        @endpoint: $endpoint:expr;
        @ret:  $ret:ty;
      }
    )+
  ) => {
    #[async_trait::async_trait]
    impl OsuApiRequester for reqwest::Client {
      $(
        async fn $name<Q>(
          &self,
          query: Q,
        ) -> Result<$ret, Error>
        where Q: TryInto<Query, Error = Error> + Send + Sync
      {
          let query: Result<Query, Error> = query.try_into();
          let url = reqwest::Url::parse(&format!("{}?{}", $endpoint, query.unwrap().into_query_str()))
            .expect(concat!("fail to parse param in ", stringify!($name)));
          let resp = self.get(url).send().await?.bytes().await?;
          let ret: $ret = serde_json::from_slice(&resp)?;

          Ok(ret)
        }
      )+
    }
  };
}

impl_reqwest! {
  get_user_recent {
    @endpoint: ApiEndpoint::GET_USER_RECENT;
    @ret: Vec<GetUserRecentResp>;
  }

  get_beatmaps {
    @endpoint: ApiEndpoint::GET_BEATMAPS;
    @ret: Vec<GetBeatmapsResp>;
  }
}

#[tokio::test]
async fn test_get_user_recent() {
  use crate::api::GetUserRecentProp;
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
  use crate::api::GetBeatmapsProps;
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
