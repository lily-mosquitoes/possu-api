use possu_api;
use rocket::local::asynchronous::Client;

pub(crate) async fn get_http_client_with_database(
    database_path: &str,
) -> Client {
    let database_url = format!(
        "sqlite://target/sqlx/test-dbs/{}.sqlite",
        database_path
    );

    Client::untracked(possu_api::rocket(&database_url))
        .await
        .expect("valid rocket instance")
}
