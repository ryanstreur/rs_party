use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract, Json};
use std::sync::Arc;

use sqlx::PgPool;

use crate::model::{NewUserParams, User};
use crate::{db, model::LoginParams};

pub struct AppState {
    pub db: PgPool,
}

pub async fn root_handler() -> String {
    "Hello World".to_string()
}

pub async fn registration_handler(
    State(state): State<Arc<AppState>>,
    extract::Json(new_user_params): extract::Json<NewUserParams>,
) -> Result<axum::Json<User>, StatusCode> {
    let mut conn = state
        .db
        .acquire()
        .await
        .expect("could not get db connection");

    let user_result = db::insert_user(&mut conn, &new_user_params).await;

    match user_result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    extract::Json(login_params): extract::Json<LoginParams>,
) -> Result<String, String> {
    let conn = state
        .db
        .acquire()
        .await
        .expect("could not get db connection");

    let user_result = db::login(conn, &login_params).await;
    match user_result {
        Ok(session_id) => Ok(session_id),
        Err(err_str) => Err(err_str),
    }
}
