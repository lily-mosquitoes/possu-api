use rocket::{
    get,
    http::Status,
    serde::json::Json,
};

use crate::{
    database::{
        self,
        Expense,
    },
    response::Response,
};

#[get("/healthcheck")]
pub(crate) fn get_healthcheck() -> (Status, Json<Response<(), ()>>) {
    let response = Response::from_result(Ok(()));

    (Status::Ok, Json(response))
}

#[get("/expense")]
pub(crate) fn get_expense_list(
) -> (Status, Json<Response<Vec<Expense>, ()>>) {
    let result = database::get_expense_list();

    let response = Response::from_result(result);

    (Status::Ok, Json(response))
}

#[get("/expense/<id>")]
pub(crate) fn get_expense(
    id: &str,
) -> (Status, Json<Response<Expense, ()>>) {
    let result = database::get_expense(id);

    let response = Response::from_result(result);

    (Status::Ok, Json(response))
}
