use super::models;

use paste::paste;

/// Generate constant endpoint
macro_rules! generate_endpoint {
  ($($ep:ident;)+) => {
    paste!{
      pub struct ApiEndpoint;
      impl ApiEndpoint {
        $(
          pub const [<$ep:upper>]: &'static str = concat!("https://osu.ppy.sh/api/", stringify!($ep));
        )+
      }
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
