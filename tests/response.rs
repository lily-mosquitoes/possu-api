use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct OkResponse<T> {
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) data: T,
}

#[derive(Deserialize)]
pub(crate) struct ErrorResponse<E> {
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) error: E,
}
