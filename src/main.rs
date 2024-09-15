use rocket::Rocket;
use routes::Routes;

mod config;
mod routes;

#[macro_use]
extern crate rocket;

#[rocket::launch]
async fn launch() -> Rocket<rocket::Build> {
    dotenv::dotenv().ok();

    rocket::custom(config::from_env()).mount_routes()
}
