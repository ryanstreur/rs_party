use rocket::request::FromRequest;
use rocket::response::status;
use rocket::{get, Request};

use rocket::http::Status;
use rocket::request::Outcome;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::PgPool;

use crate::model::User;
use crate::{db, db::AppDb, model, model::LoginParams};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/first-user")]
pub async fn first_user(db: Connection<AppDb>) -> Result<Json<model::User>, (Status, String)> {
    let user_result = db::get_first_user(db).await;

    match user_result {
        Ok(user) => Ok(Json(user)),
        Err(sqlx_err) => Err((Status::InternalServerError, sqlx_err.to_string())),
    }
}

#[get("/register")]
pub async fn register(db: Connection<AppDb>) -> Result<Json<model::User>, (Status, String)> {
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

#[derive(Debug)]
struct AppError {
    message: String,
}

// impl <'r> FromRequest<'r> for AppDb {
//   type Error = AppError;

//   fn from_request(_: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//     Outcome::Success(Self(PgPool))
//   }
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for User {
//     type Error = AppError;

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         // Outcome::Success(RequestRef { req: &request })

//         let user_id_option = request.headers().get_one("Authorization");

//         let user_id_str = match user_id_option {
//             Some(id) => id,
//             None => {
//                 return Outcome::Error((
//                     Status::Unauthorized,
//                     AppError {
//                         message: "No Authorization header".to_string(),
//                     },
//                 ));
//             }
//         };

//         let user_id_parse_result = user_id_str.parse::<i64>();

//         let user_id = match user_id_parse_result {
//           Ok(id) => id,
//           Err(_) => {
//             return Outcome::Error((
//               status::BadRequest,
//               AppError {
//                 message: "Could not parse User Id from Header".to_string()
//               }
//             ))
//           }
//         };

//         let db = AppDb::from_request(request);

//     }
// }

#[get("/login", data = "<login_params_in>")]
pub async fn login(
    db: Connection<AppDb>,
    login_params_in: Json<LoginParams>,
) -> Result<String, String> {
    let login_params = login_params_in.into_inner();

    // let login_params = match login_params_result {
    //     Ok(params) => params,
    //     Err(e) => return Err((Status::BadRequest, "Error parsing login params".to_string())),
    // };

    let user_result = db::login(db, &login_params).await;

    // db::log_request(db, req);

    match user_result {
        Ok(session_id) => Ok(session_id),
        Err(err_str) => Err(err_str),
    }
}
