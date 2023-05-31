use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
pub(crate) struct OkResponse<T> {
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) data: T,
}

#[allow(unused)]
#[derive(Deserialize)]
pub(crate) struct ErrorResponse<E> {
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) error: E,
}
