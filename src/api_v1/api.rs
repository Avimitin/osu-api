use super::models;
use crate::api_v1::req::Query;

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

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("the given request param is invalid")]
  InvalidRequestParams,
  #[error("fail to send request")]
  NetIO(#[from] reqwest::Error),
  #[error("fail to deserialize response into expected type")]
  UnexpectedResponse(#[from] serde_json::Error),
}

macro_rules! generate_trait {
  ( $( $name:ident -> $ret:ty ),+ ) => {
    #[async_trait::async_trait]
    pub trait OsuApiRequester {
      $(
        async fn $name<Q>(&self, query: Q) -> Result<$ret, Error>
          where Q: TryInto<Query, Error = Error> + Send + Sync;
      )+
    }
  }
}

generate_trait! {
  get_user_recent -> Vec<models::GetUserRecentResp>,
  get_beatmaps    -> Vec<models::GetBeatmapsResp>
}
