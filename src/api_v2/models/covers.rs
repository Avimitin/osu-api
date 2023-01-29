use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Covers {
    pub list: String,
    pub cover: String,
    pub card: String,
    pub slimcover: String,
    #[serde(rename = "list@2x")]
    pub list2x: String,
    #[serde(rename = "cover@2x")]
    pub cover2x: String,
    #[serde(rename = "card@2x")]
    pub card2x: String,
    #[serde(rename = "slimcover@2x")]
    pub slimcover2x: String,
}