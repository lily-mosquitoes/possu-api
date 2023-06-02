use dotenvy::dotenv;
use possu_api::rocket;
use rocket::launch;

#[launch]
fn launch() -> _ {
    dotenv().expect(".env file not found");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    rocket(&database_url)
}
