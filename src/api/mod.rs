use rocket::{
    get,
    http::Status,
    serde::json::Json,
};
use rocket_db_pools::Connection;

use crate::{
    database::{
        self,
        setup::PossuDatabase,
        DatabaseError,
        Entry,
    },
    response::Response,
};

#[get("/healthcheck")]
pub(crate) fn get_healthcheck() -> (Status, Json<Response<(), ()>>) {
    let response = Response::from_result(Ok(()));

    (Status::Ok, Json(response))
}

#[get("/entry")]
pub(crate) async fn get_entry_list(
    mut connection: Connection<PossuDatabase>,
) -> (Status, Json<Response<Vec<Entry>, DatabaseError>>) {
    let result = database::get_entry_list(&mut **connection).await;

    let status = match result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    };

    let response = Response::from_result(result);

    (status, Json(response))
}

#[get("/entry/<id>")]
pub(crate) async fn get_entry(
    id: i64,
    mut connection: Connection<PossuDatabase>,
) -> (Status, Json<Response<Entry, DatabaseError>>) {
    let result = database::get_entry(id, &mut **connection).await;

    let status = match result {
        Ok(_) => Status::Ok,
        Err(DatabaseError::NotFound) => Status::NotFound,
        Err(DatabaseError::InternalServerError) => {
            Status::InternalServerError
        },
    };

    let response = Response::from_result(result);

    (status, Json(response))
}
