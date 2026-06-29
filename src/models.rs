use serde::{Deserialize, Serialize};
use sqlx::{postgres::types::PgInterval, FromRow};
use uuid::Uuid;
use chrono::NaiveDateTime;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Duration {
    pub months: i32,
    pub days: i32,
    pub microseconds: i64,
}