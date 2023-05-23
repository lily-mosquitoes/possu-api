mod client;
mod response;

use chrono::{
    DateTime,
    Utc,
};
use client::get_tracked_client;
use response::OkResponse;
use rocket::http::{
    uri::Origin,
    Status,
};
use serde::Deserialize;

static TEST_URI: &str = "/api/expense";

#[derive(Deserialize)]
struct Expense {
    timestamp: DateTime<Utc>,
    category: String,
    description: String,
    value: u64,
}

fn register_expense() -> String {
    "0".to_string()
}

fn deregister_expense(_id: String) {}

#[test]
fn get_expense_returns_status_ok() {
    let client = get_tracked_client();
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_expense_returns_expenses_vec_ok_response() {
    let client = get_tracked_client();
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch();
    let body = response.into_json::<OkResponse<Vec<Expense>>>();
    assert!(body.is_some());
}

#[test]
fn get_expense_with_id_returns_status_ok_if_exists() {
    let client = get_tracked_client();
    let valid_expense_id = register_expense();
    let test_uri = format!("{}/{}", TEST_URI, valid_expense_id);
    let uri = Origin::parse(&test_uri).expect("URI to be valid");
    let response = client.get(uri).dispatch();
    deregister_expense(valid_expense_id);
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_expense_with_id_returns_expense_ok_response_if_exists() {
    let client = get_tracked_client();
    let valid_expense_id = register_expense();
    let test_uri = format!("{}/{}", TEST_URI, valid_expense_id);
    let uri = Origin::parse(&test_uri).expect("URI to be valid");
    let response = client.get(uri).dispatch();
    deregister_expense(valid_expense_id);
    let body = response.into_json::<OkResponse<Expense>>();
    assert!(body.is_some());
}
