//! Main module
pub mod conf;
pub mod db;
pub mod logging;
pub mod model;

use model::NewUserParams;
use rocket::response::status;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn users() -> status::Accepted<Json<model::User>> {
    let res = model::User {
        id: 0,
        name: "test user".to_string(),
        email: "test@example.com".to_string(),
        is_superuser: false,
    };

    status::Accepted(Json(res))
}

#[post("/users/new", format="application/json", data="<user_in>")]
fn new_user(user_in: NewUserParams) -> status::Accepted<Json<model::User>> {
  
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/users", routes![users])
}
