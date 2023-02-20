use std::sync::{Arc, Mutex};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use crate::api_v2::{BeatMap, Error, User};
use crate::api_v2::Error::NetworkErr;

#[derive(Default)]
pub struct Scopes {
    scopes_str: String,
}

impl Scopes {
    pub fn builder() -> Self {
        Self::default()
    }

    /// Allows sending chat messages on a user's behalf.
    ///
    /// Using the chat.write scope requires either:
    /// - a Chat Bot account to send messages on behalf of other users.
    /// - Authorization code grant where the user is the same as the client's owner (send as yourself).
    pub fn chat(self) -> Self {
        self.add_scope("chat.write")
    }

    /// Allows acting as the owner of a client; only available for Client Credentials Grant.
    pub fn delegate(self) -> Self {
        self.add_scope("delegate")
    }

    /// Allows creating and editing forum posts on a user's behalf.
    pub fn forum(self) -> Self {
        self.add_scope("forum.write")
    }

    /// Allows reading of the user's friend list.
    pub fn friends(self) -> Self {
        self.add_scope("friends.read")
    }

    ///
    /// Allows reading of the public profile of the user (/me).
    ///
    /// `identify`  is the default scope for the Authorization Code Grant and always implicitly provided.
    pub fn identify(self) -> Self {
        self.add_scope("identify")
    }

    /// Allows reading of publicly available data on behalf of the user.
    pub fn public(self) -> Self {
        self.add_scope("public")
    }

    fn add_scope(mut self, scope: &str) -> Self {
        let s = &self.scopes_str;
        self.scopes_str = format!("{s} {scope}");
        self
    }

    fn create(self) -> String {
        self.scopes_str
    }
}

pub mod api_url {
    const API_URL: &str = "https://osu.ppy.sh/oauth/";

    pub fn get_authorize() -> String {
        format!("{API_URL}/authorize")
    }

    pub fn get_token() -> String {
        format!("{API_URL}/token")
    }

    pub fn get_beatmap(bid: i64) -> String {
        format!("{API_URL}/beatmaps/{bid}")
    }
}

pub struct Api {
    bot: Arc<Mutex<User>>,
    client: reqwest::Client,
    redirect_uri: String,
}

impl Api {
    /// url is application callback url
    /// make sure callback url right
    pub fn new(client_id: i32, code: String, url: String) -> Result<Self, Error> {
        let bot = User::create_bot(code, client_id);
        let bot = Arc::new(Mutex::new(bot));
        let client = reqwest::Client::new();
        let redirect_uri = url;
        Ok(Self {
            bot,
            client,
            redirect_uri,
        })
    }

    pub fn get_oauth_url(&self, scopes: Scopes, state: &str) -> String {
        let bot_user = self.bot.lock().unwrap();
        let client_id = bot_user.uid;
        let redirect_uri = &self.redirect_uri;
        let response_type = "code";
        let scope = scopes.create();

        let url = api_url::get_authorize();
        format!(
            "{url}?\
            client_id={client_id}&\
            redirect_uri={redirect_uri}&\
            response_type={response_type}&\
            scope={scope}&\
            state={state}")
    }

    pub async fn refresh_token(&self, mut user: User) -> (Result<(), Error>, User) {
        let url = api_url::get_token();
        let header = self.header(&user);

        let body;
        {
            let bot_user = self.bot.lock().unwrap();
            body = json!({
                "client_id": bot_user.uid.to_string(),
                "client_secret": bot_user.refresh_token.to_string(),
                "refresh_token": user.refresh_token.to_string(),
                "grant_type": "refresh_token".to_string(),
                "redirect_uri": self.redirect_uri.clone()
            }).to_string();
        }

        let rep = self.client.post(url)
            .headers(header)
            .body(body)
            .send().await;

        if let Err(e) = rep {
            return (Err(NetworkErr(e.to_string())), user);
        }
        let rep = rep.unwrap();
        let str = rep.text().await.unwrap();
        let json = json!(str);
        let access_token = json["access_token"].as_str().unwrap();
        let refresh_token = json["refresh_token"].as_str().unwrap();
        let expires_in = json["expires_in"].as_i64().unwrap();

        user.access_token = access_token.to_string();
        user.refresh_token = refresh_token.to_string();
        user.next_time(expires_in as u64);

        (Ok(()), user)
    }

    /// make sure user's access_token is alive
    fn header(&self, user: &User) -> HeaderMap {
        let mut head = HeaderMap::new();

        if let Some(e) = user.get_access_token() {
            head.insert("Authorization", HeaderValue::from_str(&format!("Bearer {e}")).unwrap());
        }
        head.insert("Content-Type", HeaderValue::from_static("application/json"));
        head.insert("Accept", HeaderValue::from_static("application/json"));
        head
    }

    pub async fn get_beatmap(&self, beatmap_id: i64, user: User) -> (Result<BeatMap, Error>, User) {
        let url = api_url::get_beatmap(beatmap_id);
        let user_alive;
        if !user.is_alive() {
            let (e, user) = self.refresh_token(user).await;
            if e.is_err() {
                return (Err(Error::Err("".to_string())), user);
            }
            user_alive = user;
        } else {
            user_alive = user;
        }
        let header = self.header(&user_alive);
        let rep = self.client
            .get(url)
            .headers(header)
            .send().await;

        if let Err(e) = rep {
            return (Err(NetworkErr(e.to_string())), user_alive);
        }

        let rep = rep.unwrap();
        let beatmap = rep.text().await.unwrap();
        let beatmap = serde_json::from_str::<BeatMap>(beatmap.as_str()).unwrap();

        (Ok(beatmap), user_alive)
    }
}