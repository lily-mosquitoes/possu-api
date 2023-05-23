mod api;
mod database;
mod response;

use rocket::{
    routes,
    Build,
    Rocket,
};

pub fn rocket() -> Rocket<Build> {
    let routes = routes![
        crate::api::get_healthcheck,
        crate::api::get_expense_list,
        crate::api::get_expense,
    ];

    rocket::build().mount("/api", routes)
}
