use chrono::{
    DateTime,
    Utc,
};
use rocket_db_pools::sqlx::SqliteConnection;
use sqlx::Acquire;

use crate::database::Entry;

pub(crate) async fn get_entry_list(
    connection: &mut SqliteConnection,
) -> Result<Vec<Entry>, sqlx::Error> {
    let result = sqlx::query_as!(
        Entry,
        r#"SELECT
            entries.id,
            timestamp AS "timestamp: DateTime<Utc>",
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
            Err(error)
        },
    }
}

pub(crate) async fn get_entry(
    id: i64,
    connection: &mut SqliteConnection,
) -> Result<Entry, sqlx::Error> {
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
        Err(sqlx::Error::RowNotFound) => {
            Err(sqlx::Error::RowNotFound)
        },
        Err(error) => {
            log::error!("{}", error);
            Err(error)
        },
    }
}

pub(crate) async fn insert_entry_transaction(
    entry: &Entry,
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        r#"INSERT INTO entries (
          timestamp,
          category_id,
          description,
          value
        ) SELECT ?, categories.id, ?, ?
        FROM categories WHERE categories.name = ?
        RETURNING id"#,
        entry.timestamp,
        entry.description,
        entry.value,
        entry.category
    )
    .fetch_one(transaction)
    .await?;

    result
        .id
        .ok_or(sqlx::Error::ColumnNotFound("id".to_string()))
}

pub(crate) async fn insert_entry_list(
    entries: &Vec<Entry>,
    connection: &mut SqliteConnection,
) -> Result<Vec<Entry>, sqlx::Error> {
    let mut created_entries = Vec::new();

    let mut transaction = connection.begin().await?;

    for entry in entries {
        let result =
            insert_entry_transaction(&entry, &mut transaction).await;
        match result {
            Ok(id) => {
                let mut new_entry = entry.clone();
                new_entry.id = id;
                created_entries.push(new_entry);
            },
            Err(error) => {
                transaction.rollback().await?;
                return Err(error);
            },
        }
    }

    transaction.commit().await?;

    Ok(created_entries)
}

#[cfg(test)]
mod test {
    use chrono::{
        DateTime,
        Utc,
    };
    use rocket_db_pools::sqlx::SqlitePool;

    use super::{
        get_entry,
        get_entry_list,
        insert_entry_list,
        insert_entry_transaction,
        Entry,
    };

    // Fixtures from tests/fixtures/entries.sql
    static EXISTING_ENTRY_ID: i64 = 345;
    static NON_EXISTING_ENTRY_ID: i64 = 678;
    static NUMBER_OF_ENTRIES: usize = 1;

    // from migrations
    static EXISTING_CATEGORY: &str = "Monthly Bills";
    static OTHER_EXISTING_CATEGORY: &str = "Yearly Bills";
    static NON_EXISTING_CATEGORY: &str = "NON EXISTING CATEGORY";

    fn new_entry_with_category(category: &str) -> Entry {
        Entry {
            id: 0,
            timestamp: Utc::now(),
            category: category.to_string(),
            description: format!("Testing - {}", category),
            value: Utc::now().timestamp(),
        }
    }

    fn new_entries_correct() -> Vec<Entry> {
        vec![
            new_entry_with_category(EXISTING_CATEGORY),
            new_entry_with_category(OTHER_EXISTING_CATEGORY),
        ]
    }

    fn new_entries_with_one_non_existing_category() -> Vec<Entry> {
        vec![
            new_entry_with_category(EXISTING_CATEGORY),
            new_entry_with_category(NON_EXISTING_CATEGORY),
        ]
    }

    #[sqlx::test]
    async fn get_entry_list_returns_empty_vec_when_zero_records(
        pool: SqlitePool,
    ) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry_list: Vec<Entry> = get_entry_list(&mut connection)
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
        let entry_list: Vec<Entry> = get_entry_list(&mut connection)
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
    fn get_entry_returns_entry_when_id_exists(pool: SqlitePool) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry: Result<Entry, sqlx::Error> =
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
        let entry: Result<Entry, sqlx::Error> =
            get_entry(NON_EXISTING_ENTRY_ID, &mut connection).await;
        assert_eq!(
            entry.map_err(|e| e.to_string()),
            Err(sqlx::Error::RowNotFound.to_string())
        );
    }

    #[sqlx::test(fixtures("drop_entries"))]
    fn get_entry_returns_other_error_on_server_failure(
        pool: SqlitePool,
    ) {
        let mut connection = pool
            .acquire()
            .await
            .expect("database connection to be acquired")
            .detach();
        let entry: Result<Entry, sqlx::Error> =
            get_entry(EXISTING_ENTRY_ID, &mut connection).await;
        let result = match entry {
            Ok(_) | Err(sqlx::Error::RowNotFound) => {
                "ok or not found error"
            },
            Err(_) => "other database error",
        };
        assert_eq!(result, "other database error");
    }

    #[sqlx::test]
    fn insert_entry_transaction_returns_ok_when_category_exists(
        pool: SqlitePool,
    ) {
        let entry = new_entry_with_category(EXISTING_CATEGORY);
        let mut transaction =
            pool.begin().await.expect("transaction to be started");
        let result =
            insert_entry_transaction(&entry, &mut transaction).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    fn insert_entry_transaction_returns_rownotfound_error_when_category_not_exists(
        pool: SqlitePool,
    ) {
        let entry = new_entry_with_category(NON_EXISTING_CATEGORY);
        let mut transaction =
            pool.begin().await.expect("transaction to be started");
        let result =
            insert_entry_transaction(&entry, &mut transaction).await;
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err(sqlx::Error::RowNotFound.to_string())
        );
    }

    #[sqlx::test]
    fn insert_entry_transaction_registers_entry_with_returned_id(
        pool: SqlitePool,
    ) {
        let mut entry = new_entry_with_category(EXISTING_CATEGORY);
        let mut transaction =
            pool.begin().await.expect("transaction to be started");
        let id = insert_entry_transaction(
            &(entry.clone()),
            &mut transaction,
        )
        .await
        .expect("result to be ok");
        transaction
            .commit()
            .await
            .expect("transaction to commit sucessfully");
        let mut connection =
            pool.acquire().await.expect("connection to be acquired");
        let registered_entry = sqlx::query_as!(
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
        .fetch_one(&mut connection)
        .await
        .expect("entry to be found");
        entry.id = id;
        assert_eq!(entry, registered_entry);
    }

    #[sqlx::test]
    fn insert_entry_list_returns_ok_when_all_ok(pool: SqlitePool) {
        let entries = new_entries_correct();
        let mut connection =
            pool.acquire().await.expect("connection to be acquired");
        let result =
            insert_entry_list(&entries, &mut connection).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    fn insert_entry_list_registers_entries_when_all_ok(
        pool: SqlitePool,
    ) {
        let entries = new_entries_correct();
        let mut connection =
            pool.acquire().await.expect("connection to be acquired");
        let _result =
            insert_entry_list(&entries, &mut connection).await;
        for entry in entries {
            let database_entry = sqlx::query!(
                r#"SELECT entries.id FROM entries
                INNER JOIN categories ON categories.id = entries.category_id
                WHERE timestamp = ?
                AND categories.name = ?
                AND description = ?
                AND value = ?
                "#,
                entry.timestamp,
                entry.category,
                entry.description,
                entry.value
            )
            .fetch_one(&mut connection)
            .await;
            assert!(database_entry.is_ok());
        }
    }

    #[sqlx::test]
    fn insert_entry_list_returns_rownotfound_error_when_one_contains_error(
        pool: SqlitePool,
    ) {
        let entries = new_entries_with_one_non_existing_category();
        let mut connection =
            pool.acquire().await.expect("connection to be acquired");
        let result =
            insert_entry_list(&entries, &mut connection).await;
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err(sqlx::Error::RowNotFound.to_string())
        );
    }

    #[sqlx::test]
    fn insert_entry_list_does_not_register_entries_when_one_contains_error(
        pool: SqlitePool,
    ) {
        let entries = new_entries_with_one_non_existing_category();
        let mut connection =
            pool.acquire().await.expect("connection to be acquired");
        let _result =
            insert_entry_list(&entries, &mut connection).await;
        let database_entries = sqlx::query_as!(
            Entry,
            r#"SELECT
                entries.id,
                timestamp as "timestamp: DateTime<Utc>",
                categories.name AS category,
                description,
                value
            FROM entries
            INNER JOIN categories ON categories.id = entries.category_id"#,
        )
        .fetch_all(&mut connection)
        .await
        .expect("query to return");
        assert_eq!(database_entries.len(), 0);
    }
}
