use chrono::{
    DateTime,
    Utc,
};
use rocket_db_pools::sqlx::SqliteConnection;

use super::{
    DatabaseError,
    Entry,
};

pub(crate) async fn get_entry_list(
    connection: &mut SqliteConnection,
) -> Result<Vec<Entry>, DatabaseError> {
    let result = sqlx::query_as!(
        Entry,
        r#"SELECT
            entries.id,
            timestamp as "timestamp: DateTime<Utc>",
            categories.name AS category,
            description,
            value
        FROM entries
        INNER JOIN categories ON categories.id = entries.category_id"#
    )
    .fetch_all(connection)
    .await;

    match result {
        Ok(entries) => Ok(entries),
        Err(error) => {
            log::error!("{}", error);
            Err(DatabaseError::InternalServerError)
        },
    }
}

pub(crate) async fn get_entry(
    id: i64,
    connection: &mut SqliteConnection,
) -> Result<Entry, DatabaseError> {
    let result = sqlx::query_as!(
        Entry,
        r#"SELECT
            entries.id,
            timestamp as "timestamp: DateTime<Utc>",
            categories.name AS category,
            description,
            value
        FROM entries
        INNER JOIN categories ON categories.id = entries.category_id
        WHERE entries.id = ?"#,
        id
    )
    .fetch_one(connection)
    .await;

    match result {
        Ok(entry) => Ok(entry),
        Err(sqlx::Error::RowNotFound) => Err(DatabaseError::NotFound),
        Err(error) => {
            log::error!("{}", error);
            Err(DatabaseError::InternalServerError)
        },
    }
}

#[cfg(test)]
mod test {
    use rocket_db_pools::sqlx::SqlitePool;

    use super::{
        get_entry,
        get_entry_list,
        DatabaseError,
        Entry,
    };

    // Fixtures from tests/fixtures/entries.sql
    static EXISTING_ENTRY_ID: i64 = 345;
    static NON_EXISTING_ENTRY_ID: i64 = 678;
    static NUMBER_OF_ENTRIES: usize = 1;

    #[sqlx::test]
    async fn get_entry_list_returns_empty_list_when_zero_records(
        pool: SqlitePool,
    ) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry_list = get_entry_list(&mut connection)
            .await
            .expect("entry list to be returned");
        assert_eq!(entry_list.len(), 0);
    }

    #[sqlx::test(fixtures("entries"))]
    async fn get_entry_list_returns_non_empty_list_when_records_exist(
        pool: SqlitePool,
    ) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry_list = get_entry_list(&mut connection)
            .await
            .expect("entry list to be returned");
        assert_eq!(entry_list.len(), NUMBER_OF_ENTRIES);
    }

    #[sqlx::test(fixtures("drop_entries"))]
    fn get_entry_list_returns_internal_server_error_on_failure(
        pool: SqlitePool,
    ) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry_list = get_entry_list(&mut connection).await;
        assert!(entry_list.is_err());
    }

    #[sqlx::test(fixtures("entries"))]
    fn get_entry_returns_expense_when_id_exists(pool: SqlitePool) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry: Result<Entry, DatabaseError> =
            get_entry(EXISTING_ENTRY_ID, &mut connection).await;
        assert!(entry.is_ok());
    }

    #[sqlx::test(fixtures("entries"))]
    fn get_entry_returns_not_found_error_when_id_not_exists(
        pool: SqlitePool,
    ) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry: Result<Entry, DatabaseError> =
            get_entry(NON_EXISTING_ENTRY_ID, &mut connection).await;
        assert_eq!(entry, Err(DatabaseError::NotFound));
    }

    #[sqlx::test(fixtures("drop_entries"))]
    fn get_entry_returns_internal_server_error_on_server_failure(
        pool: SqlitePool,
    ) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry: Result<Entry, DatabaseError> =
            get_entry(EXISTING_ENTRY_ID, &mut connection).await;
        assert_eq!(entry, Err(DatabaseError::InternalServerError));
    }
}
