mod common;

use common::{
    client,
    response::OkResponse,
};
use rocket::http::{
    uri::Origin,
    Status,
};
use sqlx::SqlitePool;

static TEST_URI: &str = "/api/healthcheck";

#[sqlx::test]
async fn returns_status_ok(_pool: SqlitePool) {
    let test_database = "get_healthcheck/returns_status_ok";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test]
async fn returns_empty_ok_response(_pool: SqlitePool) {
    let test_database = "get_healthcheck/returns_empty_ok_response";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    let body = response.into_json::<OkResponse<()>>().await;
    assert!(body.is_some());
}
