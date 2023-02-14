use std::collections::HashMap;
use std::sync::Arc;
use serde_json::json;
use crate::api_v2::{Error, User};

#[derive(Default)]
pub struct Scopes {
    str: String,
}

impl Scopes {
    pub fn new() -> Self {
        return Self::default();
    }

    /// Allows sending chat messages on a user's behalf.
    ///
    /// Using the chat.write scope requires either:
    /// - a Chat Bot account to send messages on behalf of other users.
    /// - Authorization code grant where the user is the same as the client's owner (send as yourself).
    pub fn chat(self) -> Self {
        return self.add_scope("chat.write");
    }

    /// Allows acting as the owner of a client; only available for Client Credentials Grant.
    pub fn delegate(self) -> Self {
        return self.add_scope("delegate");
    }

    /// Allows creating and editing forum posts on a user's behalf.
    pub fn forum(self) -> Self {
        return self.add_scope("forum.write");
    }

    /// Allows reading of the user's friend list.
    pub fn friends(self) -> Self {
        return self.add_scope("friends.read");
    }

    ///
    /// Allows reading of the public profile of the user (/me).
    ///
    /// `identify`  is the default scope for the Authorization Code Grant and always implicitly provided.
    pub fn identify(self) -> Self {
        return self.add_scope("identify");
    }

    /// Allows reading of publicly available data on behalf of the user.
    pub fn public(self) -> Self {
        return self.add_scope("public");
    }

    fn add_scope(mut self, scope: &str) -> Self {
        let s = &self.str;
        self.str = format!("{s} {scope}");
        return self;
    }

    fn to_string(self) -> String {
        return self.str.to_string();
    }
}

pub mod api_url {
    const API_URL: &str = "https://osu.ppy.sh/oauth/";

    pub fn get_authorize() -> String {
        return format!("{API_URL}/authorize");
    }

    pub fn get_token() -> String {
        return format!("{API_URL}/token");
    }
}

pub struct Api {
    bot: Arc<User>,
    client: reqwest::Client,
    redirect_uri: String,
}

impl Api {
    /// url is application callback url
    /// make sure callback url right
    pub fn new(client_id: i32, code: String, url: String) -> Result<Self, Error> {
        let bot = User::create_bot(code, client_id);
        let bot = Arc::new(bot);
        let client = reqwest::Client::new();
        let redirect_uri = url.to_string();
        Ok(Self {
            bot,
            client,
            redirect_uri,
        })
    }

    pub fn get_oauth_url(&self, scopes: Scopes, state: &str) -> String {
        let client_id = &self.bot.uid;
        let redirect_uri = &self.redirect_uri;
        let response_type = "code";
        let scope = scopes.to_string();

        let mut url = api_url::get_authorize();
        url = format!(
            "{url}?\
            client_id={client_id}&\
            redirect_uri={redirect_uri}&\
            response_type={response_type}&\
            scope={scope}&\
            state={state}");
        return url;
    }

    pub async fn refresh_token(&self, user: &mut User) {
        let url = api_url::get_token();
        let header = self.header(user);
        let body = json!({
            "client_id": self.bot.uid.to_string(),
            "client_secret": self.bot.refresh_token.to_string(),
            "refresh_token": user.refresh_token.to_string(),
            "grant_type": "refresh_token".to_string(),
            "redirect_uri": self.redirect_uri.clone()
        });

        let mut request_build = self.client.post(url)
            .body(body.to_string());
        for (k, v) in header {
            request_build = request_build.header(k, v);
        }
        let rep = request_build.send().await.unwrap();
        let str = rep.text().await.unwrap();
        let json = json!(str);
        let access_token = json["access_token"].as_str().unwrap();
        let refresh_token = json["refresh_token"].as_str().unwrap();
        let expires_in = json["expires_in"].as_i64().unwrap();

        user.access_token = access_token.to_string();
        user.refresh_token = refresh_token.to_string();
        user.next_time(expires_in as u64);
    }

    /// make sure user's access_token is alive
    fn header(&self, user: &User) -> HashMap<&'static str, String> {
        let mut head = HashMap::<&'static str, String>::new();

        if let Some(e) = user.get_access_token() {
            head.insert("Authorization", format!("Bearer {}", e));
        }
        head.insert("Content-Type", "application/json".to_string());
        head.insert("Accept", "application/json".to_string());
        return head;
    }
}