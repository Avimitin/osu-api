mod models;
use std::time::Duration;

use paste::paste;

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
  key: String,
  http: reqwest::Client,
}

impl OsuApi {
  pub fn new(key: impl ToString) -> Self {
    Self {
      key: key.to_string(),
      http: reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap(),
    }
  }

  pub async fn get_beatmaps(&self, param: models::GetBeatmapsProps) {
    let param = param.into_query_param(&self.key);
    let url = reqwest::Url::parse_with_params(API_GET_BEATMAPS, &param).unwrap();
    self.http.get(url).send().await;
  }
}
