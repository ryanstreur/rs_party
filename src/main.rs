//! Main module

use rocket_db_pools::Database;

use rs_party::db::AppDb;
// use rs_party::fairings::RequestLoggerFairing;
use rs_party::routes::{first_user, index, login, register};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AppDb::init())
        // .attach(RequestLoggerFairing)
        .mount("/", routes![index, first_user, register, login])
}
