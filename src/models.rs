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
impl Default for Duration {
    fn default() -> Self {
        Duration {
            months: 0,          // Default value for months
            days: 0,            // Default value for days
            microseconds: 0,    // Default value for microseconds
        }
    }
}

impl From<PgInterval> for Duration {
    fn from(interval: PgInterval) -> Self {
        Duration {
            months: interval.months,
            days: interval.days,
            microseconds: interval.microseconds,
        }
    }
}

impl From<Option<PgInterval>> for Duration {
    fn from(option: Option<PgInterval>) -> Self {
        match option {
            Some(interval) => Duration::from(interval),
            None => Duration::default(), // Return default Duration if None
        }
    }
}