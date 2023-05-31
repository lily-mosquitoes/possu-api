mod functions;
pub(crate) mod setup;

use chrono::{
    DateTime,
    Utc,
};
pub(crate) use functions::{
    get_entry,
    get_entry_list,
};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, sqlx::FromRow)]
pub(crate) struct Entry {
    id: i64,
    timestamp: DateTime<Utc>,
    category: String,
    description: String,
    value: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub(crate) enum DatabaseError {
    NotFound,
    InternalServerError,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Resource not found"),
            Self::InternalServerError => {
                write!(f, "Internal Server Error")
            },
        }
    }
}

impl std::error::Error for DatabaseError {}
