use chrono::{
    DateTime,
    Utc,
};
use rocket::{
    catch,
    get,
    http::Status,
    post,
    serde::json::{
        Error as JsonError,
        Json,
    },
};
use rocket_db_pools::Connection;
use serde::Deserialize;

use crate::{
    database::{
        self,
        setup::PossuDatabase,
        Entry,
    },
    response::{
        IntoStatus,
        Response,
    },
};

#[catch(default)]
pub(crate) async fn default_catcher(
    status: Status,
    _: &rocket::Request<'_>,
) -> (Status, Json<Response<(), String>>) {
    (status, Json(Response::from_result(Err(status.to_string()))))
}

#[get("/healthcheck")]
pub(crate) async fn get_healthcheck(
) -> (Status, Json<Response<(), ()>>) {
    let response = Response::from_result(Ok(()));

    (Status::Ok, Json(response))
}

#[get("/entry")]
pub(crate) async fn get_entry_list(
    mut connection: Connection<PossuDatabase>,
) -> (Status, Json<Response<Vec<Entry>, String>>) {
    let result = database::get_entry_list(&mut **connection).await;

    let (status, result) = match result {
        Ok(entries) => (Status::Ok, Ok(entries)),
        Err(sqlx_error) => {
            let (status, error) = sqlx_error.into_status();
            (status, Err(error))
        },
    };

    let response = Response::from_result(result);

    (status, Json(response))
}

#[get("/entry/<id>")]
pub(crate) async fn get_entry(
    id: i64,
    mut connection: Connection<PossuDatabase>,
) -> (Status, Json<Response<Entry, String>>) {
    let result = database::get_entry(id, &mut **connection).await;

    let (status, result) = match result {
        Ok(entry) => (Status::Ok, Ok(entry)),
        Err(sqlx_error) => {
            let (status, error) = sqlx_error.into_status();
            (status, Err(error))
        },
    };

    let response = Response::from_result(result);

    (status, Json(response))
}

#[derive(Debug, Deserialize)]
pub(crate) struct EntryForm {
    #[serde(flatten)]
    pub(crate) entry: Entry,
    pub(crate) repeat: Option<Vec<DateTime<Utc>>>,
}

impl EntryForm {
    fn into_entries(self) -> Vec<Entry> {
        let mut entries = vec![self.entry.clone()];

        if let Some(dates) = self.repeat {
            for date in dates {
                let mut entry = self.entry.clone();
                entry.timestamp = date;
                entries.push(entry);
            }
        }

        entries
    }
}

#[post("/entry", format = "json", data = "<entry_form>")]
pub(crate) async fn post_entry(
    entry_form: Result<Json<EntryForm>, JsonError<'_>>,
    mut connection: Connection<PossuDatabase>,
) -> (Status, Json<Response<Vec<Entry>, String>>) {
    let (status, result) = match entry_form {
        Ok(json_entry) => {
            let result = database::insert_entry_list(
                &json_entry.into_inner().into_entries(),
                &mut **connection,
            )
            .await;

            match result {
                Ok(entries) => (Status::Created, Ok(entries)),
                Err(sqlx_error) => {
                    let (status, error) = sqlx_error.into_status();
                    (status, Err(error))
                },
            }
        },
        Err(json_error) => {
            let (status, error) = json_error.into_status();
            (status, Err(error))
        },
    };

    let response = Response::from_result(result);

    (status, Json(response))
}

#[cfg(test)]
mod test {
    use chrono::{
        DateTime,
        Utc,
    };

    use super::EntryForm;
    use crate::database::Entry;

    fn timestamp_from(iso_string: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(iso_string)
            .expect("timestamp to parse successfully")
            .into()
    }

    fn new_entry_form_with_repeat() -> (EntryForm, Vec<Entry>) {
        let entry_form = EntryForm {
            entry: Entry {
                id: 0,
                timestamp: timestamp_from("2020-10-05T14:48:00.000Z"),
                category: "Monthly Bills".to_string(),
                description: "Testing".to_string(),
                value: 280000,
            },
            repeat: Some(vec![
                timestamp_from("2020-11-05T14:48:00.000Z"),
                timestamp_from("2020-12-05T14:48:00.000Z"),
                timestamp_from("2021-01-05T14:48:00.000Z"),
            ]),
        };

        let expected_entries = vec![
            Entry {
                id: 0,
                timestamp: timestamp_from("2020-10-05T14:48:00.000Z"),
                category: "Monthly Bills".to_string(),
                description: "Testing".to_string(),
                value: 280000,
            },
            Entry {
                id: 0,
                timestamp: timestamp_from("2020-11-05T14:48:00.000Z"),
                category: "Monthly Bills".to_string(),
                description: "Testing".to_string(),
                value: 280000,
            },
            Entry {
                id: 0,
                timestamp: timestamp_from("2020-12-05T14:48:00.000Z"),
                category: "Monthly Bills".to_string(),
                description: "Testing".to_string(),
                value: 280000,
            },
            Entry {
                id: 0,
                timestamp: timestamp_from("2021-01-05T14:48:00.000Z"),
                category: "Monthly Bills".to_string(),
                description: "Testing".to_string(),
                value: 280000,
            },
        ];

        (entry_form, expected_entries)
    }

    fn new_entry_form_without_repeat() -> (EntryForm, Vec<Entry>) {
        let entry_form = EntryForm {
            entry: Entry {
                id: 0,
                timestamp: timestamp_from("2020-10-05T14:48:00.000Z"),
                category: "Monthly Bills".to_string(),
                description: "Testing".to_string(),
                value: 280000,
            },
            repeat: None,
        };

        let expected_entries = vec![Entry {
            id: 0,
            timestamp: timestamp_from("2020-10-05T14:48:00.000Z"),
            category: "Monthly Bills".to_string(),
            description: "Testing".to_string(),
            value: 280000,
        }];

        (entry_form, expected_entries)
    }

    #[test]
    fn entry_form_with_repeat_can_become_vec_of_entries() {
        let (entry_form, expected_entries) =
            new_entry_form_with_repeat();
        let entries = entry_form.into_entries();
        assert_eq!(entries, expected_entries);
    }

    #[test]
    fn entry_form_without_repeat_can_become_vec_of_entries() {
        let (entry_form, expected_entries) =
            new_entry_form_without_repeat();
        let entries = entry_form.into_entries();
        assert_eq!(entries, expected_entries);
    }
}
