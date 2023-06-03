mod api;
mod database;
mod response;

use rocket::{
    catchers,
    routes,
    Build,
    Rocket,
};
use rocket_db_pools::Database;

use crate::database::setup::PossuDatabase;

pub fn rocket(database_url: &str) -> Rocket<Build> {
    let catchers = catchers![crate::api::default_catcher];

    let routes = routes![
        crate::api::get_healthcheck,
        crate::api::get_entry_list,
        crate::api::get_entry,
        crate::api::post_entry,
        crate::api::delete_entry,
    ];

    let figment = rocket::Config::figment().merge((
        "databases.possu_database",
        rocket_db_pools::Config {
            url: database_url.into(),
            min_connections: None,
            max_connections: 10,
            connect_timeout: 3,
            idle_timeout: None,
        },
    ));

    rocket::custom(figment)
        .attach(PossuDatabase::init())
        .attach(PossuDatabase::migrate())
        .register("/", catchers)
        .mount("/api", routes)
}
