mod client;
mod response;

use client::get_tracked_client;
use response::OkResponse;
use rocket::http::{
    uri::Origin,
    Status,
};

static TEST_URI: &str = "/api/healthcheck";

#[test]
fn get_healthcheck_returns_status_ok() {
    let client = get_tracked_client();
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_healthcheck_returns_empty_ok_response() {
    let client = get_tracked_client();
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch();
    let body = response.into_json::<OkResponse<()>>();
    assert!(body.is_some());
}
