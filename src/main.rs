//! Main module

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};

use rs_party::db;
use rs_party::db::AppDb;
use rs_party::model;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/first_user")]
async fn first_user(db: Connection<AppDb>) -> Result<Json<model::User>, (Status, String)> {
    let user_result = db::get_first_user(db).await;

    match user_result {
        Ok(user) => Ok(Json(user)),
        Err(sqlx_err) => Err((Status::InternalServerError, sqlx_err.to_string())),
    }
}

#[get("/register")]
async fn register(db: Connection<AppDb>) -> Result<Json<model::User>, (Status, String)> {
    let new_user = model::NewUserParams {
        name: "New User".to_string(),
        email: "address@example.com".to_string(),
        password: "Some_weak_password_here".to_string(),
    };

    let user_result = db::insert_user(db, &new_user).await;

    match user_result {
        Ok(user) => Ok(Json(user)),
        Err(sqlx_err) => Err((Status::InternalServerError, sqlx_err.to_string())),
    }
}


// #[post("/users/new", format="application/json", data="<user_in>")]
// fn new_user(user_in: NewUserParams) -> status::Accepted<Json<model::User>> {

// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AppDb::init())
        .mount("/", routes![index, first_user, register])
}
