use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
pub(crate) struct Entry {
    pub(crate) id: i64,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) category: String,
    pub(crate) description: String,
    pub(crate) value: i64,
}
