#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
mod helper;

mod controllers;
mod models;
mod services;

use rocket::fs::{relative, FileServer};

use rocket_sync_db_pools::{database, diesel};

#[database("diesel")]
struct Db(diesel::SqliteConnection);

#[get("/")] //TODO link
fn index() -> &'static str {
    "
    API Files Missing
    "
}

#[launch]
fn rocket() -> _ {
    let base = rocket::build()
        //.mount("/", routes![index])
        .mount("/", FileServer::from(relative!("wwwroot")))
        .attach(controllers::stage())
        .attach(services::stage());

    attach_dev_cors(base)
}

#[cfg(debug_assertions)]
fn attach_dev_cors(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    use rocket_cors::{AllowedOrigins, CorsOptions};

    let allowed = AllowedOrigins::some_exact(&[
        "http://127.0.0.1:3000",
        "http://localhost:3000",
    ]);

    let cors = CorsOptions {
        allowed_origins: allowed,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("failed to create CORS fairing");

    rocket.attach(cors)
}

#[cfg(not(debug_assertions))]
fn attach_dev_cors(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
}
