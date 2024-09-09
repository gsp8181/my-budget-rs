#[macro_use]
extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
mod api;
mod bank;
mod store;
mod structs;
pub mod helper;

use rocket::fs::{relative, FileServer};
use rocket::http::uri::Absolute;

// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:5540");


use rocket_sync_db_pools::{database, diesel};

#[database("diesel")]
struct Db(diesel::SqliteConnection);


#[launch]
fn rocket() -> _ {
    rocket::build()
        //.mount("/", routes![index, upload, delete, retrieve])
        .mount("/", FileServer::from(relative!("wwwroot")))
        .attach(store::stage())
        .attach(api::stage())
        .attach(bank::stage())
}
