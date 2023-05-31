mod common;

use common::{
    client,
    database::Entry,
    response::{
        ErrorResponse,
        OkResponse,
    },
};
use rocket::{
    http::{
        uri::Origin,
        Status,
    },
    serde::json::serde_json::{
        json,
        Value,
    },
};
use sqlx::SqlitePool;

static TEST_URI: &str = "/api/entry";

fn new_entry_without_repeat() -> Value {
    json!({
        "timestamp": "2020-10-05T14:48:00.000Z",
        "category_id": 0,
        "description": "Testing",
        "value": 280000,
    })
}

fn new_entry_with_repeat() -> (Value, usize) {
    let body = json!({
        "timestamp": "2020-10-05T14:48:00.000Z",
        "category_id": 0,
        "description": "Testing",
        "value": 280000,
        "repeat": [
            "2020-11-05T14:48:00.000Z",
            "2020-12-05T14:48:00.000Z",
            "2021-01-05T14:48:00.000Z",
        ]
    });

    let expected_len = 4;

    (body, expected_len)
}

fn new_entry_wrong_semantics() -> Value {
    json!({
        "data": "hello there"
    })
}

fn new_entry_malformed() -> String {
    "{dskank;".to_string()
}

// WITHOUT REPEAT TESTS
#[sqlx::test]
#[ignore]
async fn returns_status_created_when_without_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_status_created_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_without_repeat();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    assert_eq!(response.status(), Status::Created);
}

#[sqlx::test]
#[ignore]
async fn returns_entry_vec_ok_response_when_without_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/\
         returns_entry_vec_ok_response_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_without_repeat();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    let body = response.into_json::<OkResponse<Vec<Entry>>>().await;
    assert!(body.is_some());
}

#[sqlx::test]
#[ignore]
async fn returns_entry_vec_with_1_len_when_without_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_entry_vec_with_1_len_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_without_repeat();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    let body = response
        .into_json::<OkResponse<Vec<Entry>>>()
        .await
        .expect("body to be Some");
    assert_eq!(body.data.len(), 1);
}

// WITH REPEAT TESTS
#[sqlx::test]
#[ignore]
async fn returns_status_created_when_with_repeat(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_created_when_with_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_with_repeat();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    assert_eq!(response.status(), Status::Created);
}

#[sqlx::test]
#[ignore]
async fn returns_entry_vec_ok_response_when_with_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_entry_vec_ok_response_when_with_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_with_repeat();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    let body = response.into_json::<OkResponse<Vec<Entry>>>().await;
    assert!(body.is_some());
}

#[sqlx::test]
#[ignore]
async fn returns_entry_vec_with_some_len_when_with_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/\
         returns_entry_vec_with_some_len_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, expected_len) = new_entry_with_repeat();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    let body = response
        .into_json::<OkResponse<Vec<Entry>>>()
        .await
        .expect("body to be Some");
    assert_eq!(body.data.len(), expected_len);
}

// WITHOUT BODY ERROR TESTS
#[sqlx::test]
#[ignore]
async fn returns_status_400_when_without_body(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_400_when_without_body";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.post(uri).dispatch().await;
    assert_eq!(response.status(), Status::BadRequest);
}

#[sqlx::test]
#[ignore]
async fn returns_string_error_response_when_without_body(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_string_error_response_when_without_body";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.post(uri).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

// WRONG BODY ERROR TESTS
#[sqlx::test]
#[ignore]
async fn returns_status_400_when_wrong_body(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_400_when_wrong_body";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_malformed();
    let response = client.post(uri).body(body).dispatch().await;
    assert_eq!(response.status(), Status::BadRequest);
}

#[sqlx::test]
#[ignore]
async fn returns_string_error_response_when_wrong_body(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_string_error_response_when_wrong_body";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_malformed();
    let response = client.post(uri).body(body).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

// WRONG SEMANTICS ERROR TESTS
#[sqlx::test]
#[ignore]
async fn returns_status_422_when_wrong_semantics(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_422_when_wrong_semantics";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_wrong_semantics();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[sqlx::test]
#[ignore]
async fn returns_string_error_response_when_wrong_semantics(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/\
         returns_string_error_response_when_wrong_semantics";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_wrong_semantics();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

// SERVER ERROR TESTS
#[sqlx::test(fixtures("drop_entries"))]
#[ignore]
async fn returns_status_500_when_server_error(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_500_when_server_error";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_without_repeat();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    assert_eq!(response.status(), Status::InternalServerError);
}

#[sqlx::test(fixtures("drop_entries"))]
#[ignore]
async fn returns_string_error_response_when_server_error(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_string_error_response_when_server_error";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_without_repeat();
    let response =
        client.post(uri).body(body.to_string()).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}
