#[macro_use]
extern crate rocket;
mod api;
mod bank;
mod store;
mod structs;
pub mod helper;

use rocket::fs::{relative, FileServer};
use rocket::http::uri::Absolute;

// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:5540");

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXAMPLE: curl --data-binary @file.txt http://localhost:8000

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        //.mount("/", routes![index, upload, delete, retrieve])
        .mount("/", FileServer::from(relative!("wwwroot")))
        .attach(api::stage())
        .attach(bank::stage())
}
