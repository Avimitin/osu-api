mod models;
use std::{collections::HashMap, time::Duration};

use paste::paste;

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

pub struct OsuApi {
  http: reqwest::Client,
}

impl OsuApi {
  pub fn new(key: impl ToString) -> Self {
    Self {
      http: reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap(),
    }
  }

  pub async fn get_user_recent<'k, 'u>(
    &self,
    param: models::GetUserRecentProp<'k, 'u>,
  ) -> Result<Vec<models::GetUserRecentResp>, Error> {
    let param: HashMap<&'static str, String> = param.into();
    let url = reqwest::Url::parse_with_params(API_GET_USER_RECENT, &param)
      .expect("fail to turn GetUserRecentProp into params");
    let resp = self.http.get(url).send().await?.bytes().await?;
    let recent: Vec<models::GetUserRecentResp> = serde_json::from_slice(&resp)?;

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
  let api = OsuApi::new(&api_key);
  let resp = api.get_user_recent(props).await.unwrap();

  assert!(!resp.is_empty())
}
