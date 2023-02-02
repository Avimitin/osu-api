mod models;
mod api;

pub use models::{User, UserSaver, BeatMap, BeatMapSet, Covers, Failtimes};
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Err,
}