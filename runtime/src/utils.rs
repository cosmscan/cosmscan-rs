use chrono::{NaiveDateTime, Utc};
use cosmoscout_models::models::event::NewEvent;

pub fn current_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

pub fn extract_events() -> Vec<NewEvent> {
    vec![]
}
