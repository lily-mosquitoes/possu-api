mod api;
mod response;

use rocket::{
    routes,
    Build,
    Rocket,
};

pub fn rocket() -> Rocket<Build> {
    let routes = routes![crate::api::healthcheck,];

    rocket::build().mount("/api", routes)
}
