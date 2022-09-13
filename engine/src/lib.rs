use chrono::{NaiveDateTime, Utc};

pub mod app;
pub mod client;
pub mod committer;
pub mod config;
pub mod errors;
pub mod fetcher;
pub mod messages;

pub fn current_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}
