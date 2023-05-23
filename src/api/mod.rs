use rocket::{
    get,
    http::Status,
    serde::json::Json,
};

use crate::response::Response;

#[get("/healthcheck")]
pub(crate) fn healthcheck() -> (Status, Json<Response<(), ()>>) {
    let response = Response::from_result(Ok(()));

    (Status::Ok, Json(response))
}
