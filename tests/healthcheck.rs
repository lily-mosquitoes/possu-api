mod client;
mod response;

use client::get_tracked_client;
use response::OkResponse;
use rocket::{
    http::{
        uri::Origin,
        Status,
    },
    uri,
};

static TEST_URI: Origin = uri!("/api/healthcheck");

#[test]
fn get_healthcheck_returns_status_ok() {
    let client = get_tracked_client();
    let response = client.get(TEST_URI.clone()).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_healthcheck_returns_json_ok_response() {
    let client = get_tracked_client();
    let response = client.get(TEST_URI.clone()).dispatch();
    let body = response.into_json::<OkResponse<()>>();
    assert!(body.is_some());
}
