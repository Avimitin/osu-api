use std::time::{SystemTime, UNIX_EPOCH};
use crate::api_v2::Error;

/// ```
/// if User::uid == 0{
///     .. //bot user
/// } else {
///     .. //user
/// };
/// ```
///
pub struct User {
    pub uid: i32,
    pub time: u64,
    pub access_token: String,
    pub refresh_token: String,
}

pub trait UserSave {
    fn save(&self, user: &User);
}

impl User {
    pub fn next_time(&mut self, add_time: u64) {
        let c;
        if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
            c = duration.as_millis();
        } else {
            c = 0;
        }
        self.time = c as u64 + add_time;
    }
    
    fn new() -> Self{
        User{
            uid: 0,
            time: 0,
            access_token: "".to_string(),
            refresh_token: "".to_string()
        }
    }

    pub fn new_bot(code: String) -> Self{
        User{
            uid: 0,
            time: 0,
            access_token: "".to_string(),
            refresh_token: code
        }
    }

    pub fn alive(&self) -> bool {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => { duration.as_millis() > self.time as u128 }
            Err(_) => { false }
        }
    }

    pub fn update<T: UserSave>(&mut self, access_token: String, refresh_token: String, next_time: u64, u:&T) {
        self.access_token = access_token;
        self.refresh_token = refresh_token;
        self.next_time(next_time);
        u.save(&self);
    }

    pub fn get_access_token(&self) -> Option<String>{
        if self.alive() {
            return Some(self.access_token.clone());
        } else {
            return None;
        }
    }
}