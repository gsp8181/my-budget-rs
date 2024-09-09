#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
mod api;
mod bank;
mod cardbalance;
mod cardheld;
mod cash;
mod debt;
mod debtto;
pub mod helper;
mod misccredit;
mod miscdebit;
mod regularcredit;
mod regularpayment;
mod store;
mod structs;
mod uncleared;

#[cfg(test)]
mod tests;

use rocket::fs::{relative, FileServer};
use rocket::http::uri::Absolute;

// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:5540");

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
        .attach(store::stage())
        .attach(api::stage())
        .attach(bank::stage())
        .attach(regularcredit::stage())
        .attach(cardbalance::stage())
        .attach(uncleared::stage())
        .attach(regularpayment::stage())
        .attach(miscdebit::stage())
        .attach(misccredit::stage())
        .attach(debtto::stage())
        .attach(debt::stage())
        .attach(cash::stage())
        .attach(cardheld::stage())
}
