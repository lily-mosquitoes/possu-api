mod common;

use common::{
    client,
    database::Entry,
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

// NO ENTRIES TESTS
#[sqlx::test]
async fn returns_status_ok_when_no_entries(_pool: SqlitePool) {
    let test_database = "get_entry/returns_status_ok_when_no_entries";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test]
async fn returns_entry_vec_ok_response_when_no_entries(
    _pool: SqlitePool,
) {
    let test_database =
        "get_entry/returns_entry_vec_ok_response_when_no_entries";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    let body = response.into_json::<OkResponse<Vec<Entry>>>().await;
    assert!(body.is_some());
}

#[sqlx::test]
async fn returns_entry_vec_with_0_len_when_no_entries(
    _pool: SqlitePool,
) {
    let test_database =
        "get_entry/returns_entry_vec_with_0_len_when_no_entries";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    let body = response
        .into_json::<OkResponse<Vec<Entry>>>()
        .await
        .expect("body to be Some");
    assert_eq!(body.data.len(), 0);
}

// SOME ENTRIES TESTS
#[sqlx::test(fixtures("entries"))]
async fn returns_status_ok_when_some_entries(_pool: SqlitePool) {
    let test_database =
        "get_entry/returns_status_ok_when_some_entries";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[sqlx::test(fixtures("entries"))]
async fn returns_entry_vec_ok_response_when_some_entries(
    _pool: SqlitePool,
) {
    let test_database =
        "get_entry/returns_entry_vec_ok_response_when_some_entries";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    let body = response.into_json::<OkResponse<Vec<Entry>>>().await;
    assert!(body.is_some());
}

#[sqlx::test(fixtures("entries"))]
async fn returns_entry_vec_with_some_len_when_some_entries(
    pool: SqlitePool,
) {
    let mut connection =
        pool.acquire().await.expect("database to be available");
    let database_entries = sqlx::query!("SELECT * FROM entries")
        .fetch_all(&mut connection)
        .await
        .expect("query to succeed");
    let test_database =
        "get_entry/returns_entry_vec_with_some_len_when_some_entries";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    let body = response
        .into_json::<OkResponse<Vec<Entry>>>()
        .await
        .expect("body to be Some");
    assert_eq!(body.data.len(), database_entries.len());
}

// ERROR TESTS
#[sqlx::test(fixtures("drop_entries"))]
async fn returns_status_500_when_error(_pool: SqlitePool) {
    let test_database = "get_entry/returns_status_500_when_error";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    assert_eq!(response.status(), Status::InternalServerError);
}

#[sqlx::test(fixtures("drop_entries"))]
async fn returns_string_error_response_when_error(_pool: SqlitePool) {
    let test_database =
        "get_entry/returns_string_error_response_when_error";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response = client.get(uri).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}
