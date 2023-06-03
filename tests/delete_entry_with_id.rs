mod common;

use common::{
    client,
    response::{
        ErrorResponse,
        OkResponse,
    },
};
use rocket::http::{
    uri::Origin,
    Status,
};
use sqlx::SqlitePool;

static TEST_URI: &str = "/api/entry";

// Fixtures from tests/fixtures/entries.sql
static EXISTING_ENTRY_ID: i64 = 345;
static NON_EXISTING_ENTRY_ID: i64 = 678;

// EXISTING ID TESTS
#[sqlx::test(fixtures("entries"))]
async fn returns_status_ok_if_exists(_pool: SqlitePool) {
    let test_database =
        "delete_entry_with_id/returns_status_ok_if_exists";
    let client =
        client::get_http_client_with_database(test_database).await;
    let test_uri = format!("{}/{}", TEST_URI, &EXISTING_ENTRY_ID);
    let uri = Origin::parse(&test_uri).expect("URI to be valid");
    let response = client.delete(uri).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test(fixtures("entries"))]
async fn returns_empty_ok_response_if_exists(_pool: SqlitePool) {
    let test_database =
        "delete_entry_with_id/returns_empty_ok_response_if_exists";
    let client =
        client::get_http_client_with_database(test_database).await;
    let test_uri = format!("{}/{}", TEST_URI, &EXISTING_ENTRY_ID);
    let uri = Origin::parse(&test_uri).expect("URI to be valid");
    let response = client.delete(uri).dispatch().await;
    let body = response.into_json::<OkResponse<()>>().await;
    assert!(body.is_some());
}

// NON EXISTING ID TESTS
#[sqlx::test(fixtures("entries"))]
async fn returns_status_404_if_not_exists(_pool: SqlitePool) {
    let test_database =
        "delete_entry_with_id/returns_status_404_if_not_exists";
    let client =
        client::get_http_client_with_database(test_database).await;
    let test_uri = format!("{}/{}", TEST_URI, &NON_EXISTING_ENTRY_ID);
    let uri = Origin::parse(&test_uri).expect("URI to be valid");
    let response = client.delete(uri).dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test(fixtures("entries"))]
async fn returns_string_error_response_if_not_exists(
    _pool: SqlitePool,
) {
    let test_database = "delete_entry_with_id/\
                         returns_string_error_response_if_not_exists";
    let client =
        client::get_http_client_with_database(test_database).await;
    let test_uri = format!("{}/{}", TEST_URI, &NON_EXISTING_ENTRY_ID);
    let uri = Origin::parse(&test_uri).expect("URI to be valid");
    let response = client.delete(uri).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

// ERROR TESTS
#[sqlx::test(fixtures("drop_entries"))]
async fn returns_status_500_when_error(_pool: SqlitePool) {
    let test_database =
        "delete_entry_with_id/returns_status_500_when_error";
    let client =
        client::get_http_client_with_database(test_database).await;
    let test_uri = format!("{}/{}", TEST_URI, &EXISTING_ENTRY_ID);
    let uri = Origin::parse(&test_uri).expect("URI to be valid");
    let response = client.delete(uri).dispatch().await;
    assert_eq!(response.status(), Status::InternalServerError);
}

#[sqlx::test(fixtures("drop_entries"))]
async fn returns_string_error_response_when_error(_pool: SqlitePool) {
    let test_database = "delete_entry_with_id/\
                         returns_string_error_response_when_error";
    let client =
        client::get_http_client_with_database(test_database).await;
    let test_uri = format!("{}/{}", TEST_URI, &EXISTING_ENTRY_ID);
    let uri = Origin::parse(&test_uri).expect("URI to be valid");
    let response = client.delete(uri).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}
