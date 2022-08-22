use chrono::{NaiveDateTime, Utc};

pub fn current_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}