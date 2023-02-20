use std::time::{SystemTime, UNIX_EPOCH};
use crate::api_v2::api::Api;
use crate::api_v2::Error;

/// ```
/// if User::uid == 0{
///     .. //bot user
/// } else {
///     .. //user
/// };
/// ```
///
#[derive(Default)]
pub struct User {
    pub uid: i32,
    pub time: u64,
    pub access_token: String,
    pub refresh_token: String,
}


impl User {
    pub fn next_time(&mut self, add_time: u64) {
        let c =
            if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
                duration.as_millis()
            } else {
                0
            };
        self.time = c as u64 + add_time;
    }

    pub fn create_bot(code: String, client_id: i32) -> Self {
        User {
            refresh_token: code,
            uid: client_id,
            ..Self::default()
        }
    }

    pub fn is_alive(&self) -> bool {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => { duration.as_millis() > self.time as u128 }
            Err(_) => { false }
        }
    }

    pub fn get_access_token(&self) -> Option<String> {
        if self.is_alive() {
            Some(self.access_token.clone())
        } else {
            None
        }
    }

    pub(crate) async fn get_access_token_and_refresh(self, api: &Api) -> (Result<String, Error>, Self) {
        let (r,u) = api.refresh_token(self).await;
        match r {
            Ok(_) => {
                (Ok(u.access_token.clone()), u)
            }
            Err(e) => {
                (Err(e), u)
            }
        }
    }
}