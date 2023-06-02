mod functions;
pub(crate) mod setup;

use chrono::{
    DateTime,
    Utc,
};
pub(crate) use functions::{
    get_entry,
    get_entry_list,
    insert_entry_list,
};
use rocket::http::Status;
use serde::{
    Deserialize,
    Serialize,
};

use crate::response::IntoStatus;

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow,
)]
pub(crate) struct Entry {
    #[serde(skip_deserializing)]
    pub(crate) id: i64,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) category: String,
    pub(crate) description: String,
    pub(crate) value: i64,
}

impl IntoStatus for sqlx::Error {
    fn into_status(self) -> (Status, String) {
        match self {
            sqlx::Error::RowNotFound => (
                Status::NotFound,
                "Database record not found".to_string(),
            ),
            _ => (
                Status::InternalServerError,
                "Internal Error: Connection with database failed"
                    .to_string(),
            ),
        }
    }
}
