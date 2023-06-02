mod common;

use common::{
    client,
    response::ErrorResponse,
};
use rocket::http::{
    uri::Origin,
    Status,
};
use sqlx::SqlitePool;

static TEST_URI: &str = "/non/existing/uri";

#[sqlx::test]
async fn returns_status_404_on_get(_pool: SqlitePool) {
    let test_database = "non_existing_uri/returns_status_404";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test]
async fn returns_string_error_response_on_get(_pool: SqlitePool) {
    let test_database =
        "non_existing_uri/returns_string_error_response";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

#[sqlx::test]
async fn returns_status_404_on_post(_pool: SqlitePool) {
    let test_database = "non_existing_uri/returns_status_404";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.post(uri).dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test]
async fn returns_string_error_response_on_post(_pool: SqlitePool) {
    let test_database =
        "non_existing_uri/returns_string_error_response";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.post(uri).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

#[sqlx::test]
async fn returns_status_404_on_put(_pool: SqlitePool) {
    let test_database = "non_existing_uri/returns_status_404";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.put(uri).dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test]
async fn returns_string_error_response_on_put(_pool: SqlitePool) {
    let test_database =
        "non_existing_uri/returns_string_error_response";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.put(uri).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

#[sqlx::test]
async fn returns_status_404_on_delete(_pool: SqlitePool) {
    let test_database = "non_existing_uri/returns_status_404";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.delete(uri).dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
}

#[sqlx::test]
async fn returns_string_error_response_on_delete(_pool: SqlitePool) {
    let test_database =
        "non_existing_uri/returns_string_error_response";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.delete(uri).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}
