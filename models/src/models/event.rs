use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::errors::DBModelError;

#[derive(Debug, Queryable, Serialize)]
pub struct Event {}
