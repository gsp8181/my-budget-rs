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
    rocket::build()
        //.mount("/", routes![index])
        .mount("/", FileServer::from(relative!("wwwroot")))
        .attach(controllers::stage())
        .attach(services::stage())
}
