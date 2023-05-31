use std::marker::PhantomData;

use log::error;
use rocket::{
    fairing::{
        self,
        Fairing,
        Info,
        Kind,
    },
    Build,
    Rocket,
};
use rocket_db_pools::{
    sqlx::SqlitePool,
    Database,
};

#[derive(Database)]
#[database("possu_database")]
pub(crate) struct PossuDatabase(SqlitePool);

pub(crate) struct Migrator<D: Database> {
    marker: PhantomData<D>,
}

impl PossuDatabase {
    pub(crate) fn migrate() -> Migrator<Self> {
        Migrator::<Self> {
            marker: PhantomData,
        }
    }
}

#[rocket::async_trait]
impl Fairing for Migrator<PossuDatabase> {
    fn info(&self) -> Info {
        Info {
            name: "Run database migrations",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(
        &self,
        rocket: Rocket<Build>,
    ) -> fairing::Result {
        if let Some(connection) = PossuDatabase::fetch(&rocket) {
            return match sqlx::migrate!().run(&**connection).await {
                Ok(_) => Ok(rocket),
                Err(error) => {
                    error!("Failed to run migrations: {}", error);
                    Err(rocket)
                },
            };
        }

        Err(rocket)
    }
}
