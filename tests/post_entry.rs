mod common;

use chrono::{
    DateTime,
    Utc,
};
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
        ContentType,
        Status,
    },
    serde::json::serde_json::{
        json,
        Value,
    },
};
use sqlx::SqlitePool;

static TEST_URI: &str = "/api/entry";

#[derive(Debug, PartialEq, sqlx::FromRow)]
struct ExpectedEntry {
    timestamp: DateTime<Utc>,
    category: String,
    description: String,
    value: i64,
}

fn timestamp_from(iso_string: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(iso_string)
        .expect("timestamp to parse successfully")
        .into()
}

fn new_entry_without_repeat() -> (Value, Vec<ExpectedEntry>) {
    let request = json!({
        "timestamp": "2020-10-05T14:48:00.000Z",
        "category": "Monthly Bills",
        "description": "Testing",
        "value": 280000,
    });

    let expected_entries = vec![ExpectedEntry {
        timestamp: timestamp_from("2020-10-05T14:48:00.000Z"),
        category: "Monthly Bills".to_string(),
        description: "Testing".to_string(),
        value: 280000,
    }];

    (request, expected_entries)
}

fn new_entry_with_repeat() -> (Value, Vec<ExpectedEntry>) {
    let body = json!({
        "timestamp": "2020-10-05T14:48:00.000Z",
        "category": "Monthly Bills",
        "description": "Testing",
        "value": 280000,
        "repeat": [
            "2020-11-05T14:48:00.000Z",
            "2020-12-05T14:48:00.000Z",
            "2021-01-05T14:48:00.000Z",
        ]
    });

    let expected_entries = vec![
        ExpectedEntry {
            timestamp: timestamp_from("2020-10-05T14:48:00.000Z"),
            category: "Monthly Bills".to_string(),
            description: "Testing".to_string(),
            value: 280000,
        },
        ExpectedEntry {
            timestamp: timestamp_from("2020-11-05T14:48:00.000Z"),
            category: "Monthly Bills".to_string(),
            description: "Testing".to_string(),
            value: 280000,
        },
        ExpectedEntry {
            timestamp: timestamp_from("2020-12-05T14:48:00.000Z"),
            category: "Monthly Bills".to_string(),
            description: "Testing".to_string(),
            value: 280000,
        },
        ExpectedEntry {
            timestamp: timestamp_from("2021-01-05T14:48:00.000Z"),
            category: "Monthly Bills".to_string(),
            description: "Testing".to_string(),
            value: 280000,
        },
    ];

    (body, expected_entries)
}

fn new_entry_wrong_semantics() -> Value {
    json!({
        "data": "hello there"
    })
}

fn new_entry_malformed() -> String {
    "{dskank;".to_string()
}

async fn query_entries(pool: SqlitePool) -> Vec<ExpectedEntry> {
    let mut connection =
        pool.acquire().await.expect("database to be available");

    let registered_entries = sqlx::query_as!(
        ExpectedEntry,
        r#"SELECT
            timestamp AS "timestamp: DateTime<Utc>",
            categories.name AS category,
            description,
            value
        FROM entries
        INNER JOIN categories ON categories.id = entries.category_id"#
    )
    .fetch_all(&mut connection)
    .await
    .expect("query to succeed");

    registered_entries
}

// WITHOUT REPEAT TESTS
#[sqlx::test]
async fn returns_status_created_when_without_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_status_created_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_without_repeat();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Created);
}

#[sqlx::test]
async fn returns_entry_vec_ok_response_when_without_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/\
         returns_entry_vec_ok_response_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_without_repeat();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    let body = response.into_json::<OkResponse<Vec<Entry>>>().await;
    assert!(body.is_some());
}

#[sqlx::test]
async fn returns_entry_vec_with_1_len_when_without_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_entry_vec_with_1_len_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_without_repeat();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    let body = response
        .into_json::<OkResponse<Vec<Entry>>>()
        .await
        .expect("body to be Some");
    assert_eq!(body.data.len(), 1);
}

#[sqlx::test]
async fn registers_entry_when_without_repeat(pool: SqlitePool) {
    let test_database =
        "post_entry/registers_entry_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, expected_entries) = new_entry_without_repeat();
    let _response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;

    let registered_entries = query_entries(pool).await;
    assert_eq!(registered_entries, expected_entries);
}

// WITH REPEAT TESTS
#[sqlx::test]
async fn returns_status_created_when_with_repeat(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_created_when_with_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_with_repeat();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Created);
}

#[sqlx::test]
async fn returns_entry_vec_ok_response_when_with_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_entry_vec_ok_response_when_with_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_with_repeat();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    let body = response.into_json::<OkResponse<Vec<Entry>>>().await;
    assert!(body.is_some());
}

#[sqlx::test]
async fn returns_entry_vec_with_some_len_when_with_repeat(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/\
         returns_entry_vec_with_some_len_when_without_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, expected_entries) = new_entry_with_repeat();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    let body = response
        .into_json::<OkResponse<Vec<Entry>>>()
        .await
        .expect("body to be Some");
    assert_eq!(body.data.len(), expected_entries.len());
}

#[sqlx::test]
async fn registers_entry_when_with_repeat(pool: SqlitePool) {
    let test_database = "post_entry/registers_entry_when_with_repeat";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, expected_entries) = new_entry_with_repeat();
    let _response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;

    let registered_entries = query_entries(pool).await;
    assert_eq!(registered_entries, expected_entries);
}

// WITHOUT BODY ERROR TESTS
#[sqlx::test]
async fn returns_status_400_when_without_body(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_400_when_without_body";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response =
        client.post(uri).header(ContentType::JSON).dispatch().await;
    assert_eq!(response.status(), Status::BadRequest);
}

#[sqlx::test]
async fn returns_string_error_response_when_without_body(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_string_error_response_when_without_body";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let response =
        client.post(uri).header(ContentType::JSON).dispatch().await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

// WRONG BODY ERROR TESTS
#[sqlx::test]
async fn returns_status_400_when_wrong_body(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_400_when_wrong_body";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_malformed();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::BadRequest);
}

#[sqlx::test]
async fn returns_string_error_response_when_wrong_body(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_string_error_response_when_wrong_body";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_malformed();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body)
        .dispatch()
        .await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

// WRONG SEMANTICS ERROR TESTS
#[sqlx::test]
async fn returns_status_422_when_wrong_semantics(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_422_when_wrong_semantics";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let body = new_entry_wrong_semantics();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[sqlx::test]
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
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}

// SERVER ERROR TESTS
#[sqlx::test(fixtures("drop_entries"))]
async fn returns_status_500_when_server_error(_pool: SqlitePool) {
    let test_database =
        "post_entry/returns_status_500_when_server_error";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_without_repeat();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::InternalServerError);
}

#[sqlx::test(fixtures("drop_entries"))]
async fn returns_string_error_response_when_server_error(
    _pool: SqlitePool,
) {
    let test_database =
        "post_entry/returns_string_error_response_when_server_error";
    let client =
        client::get_http_client_with_database(test_database).await;
    let uri = Origin::parse(TEST_URI).expect("URI to be valid");
    let (body, _) = new_entry_without_repeat();
    let response = client
        .post(uri)
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch()
        .await;
    let body = response.into_json::<ErrorResponse<String>>().await;
    assert!(body.is_some());
}
