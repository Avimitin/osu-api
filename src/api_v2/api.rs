use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;
use bytes::Bytes;
use reqwest::header::HeaderMap;
use crate::api_v2::{BeatMap, Error, User};

pub struct Api {
    bot: Arc<User>,
    client: reqwest::Client,
}

impl Api {
    pub fn new(client_id: String, code: String, url: String) -> Result<Self, Error> {
        let mut bot = User::new_bot(code);
        bot.uid = client_id.parse::<i32>()?;
        let bot = Arc::new(bot);
        let client = reqwest::Client::new();
        Ok(Self {
            bot,
            client,
        })
    }
}


fn header(user: &User) -> HashMap<String, String> {
    let mut head = HashMap::<String, String>::new();

    if let Some(e) = user.get_access_token() {
        head.insert("Authorization".to_string(), format!("Bearer {}", e));
    }
    head.insert("Content-Type".to_string(), "application/json".to_string());
    head.insert("Accept".to_string(), "application/json".to_string());
    return head;
}