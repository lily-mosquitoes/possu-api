use possu_api;
use rocket::local::blocking::Client;

pub(crate) fn get_tracked_client() -> Client {
    Client::tracked(possu_api::rocket())
        .expect("valid rocket instance")
}
