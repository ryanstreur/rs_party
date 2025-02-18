extern crate bcrypt;
use chrono::{Date, DateTime, TimeDelta, Utc};
use rocket::Request;
use rocket_db_pools::sqlx::PgPool;
use rocket_db_pools::{Connection, Database};

use rocket_db_pools::sqlx;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use crate::model::{LoginParams, NewUserParams, RequestLogEntry, Session, User};

const SESSION_EXPIRE_TIME: TimeDelta = TimeDelta::days(2);

#[derive(Database)]
#[database("rs_party")] // Maps to key under 'default.databases' in Rocket.toml

pub struct AppDb(PgPool);

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/rs_party".to_string());

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_connection_str)
        .await
}

pub async fn get_first_user(mut db: Connection<AppDb>) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM rs_party.user LIMIT 1;"#)
        .fetch_one(&mut **db)
        .await
}

pub async fn get_all_users(mut db: Connection<AppDb>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM rs_party.user"#)
        .fetch_all(&mut **db)
        .await
}

pub async fn insert_user(
    mut db: Connection<AppDb>,
    new_user: &NewUserParams,
) -> Result<User, sqlx::Error> {
    // Create a secure hash with the password

    let hashed_password = match bcrypt::hash(&new_user.password, bcrypt::DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => "Garbage".to_string(),
    };

    sqlx::query_as::<_, User>(r#"INSERT INTO rs_party.user (email_address, name, password) VALUES ( $1, $2, $3 ) RETURNING id, email_address, name, is_superuser;"#
    ).bind(&new_user.email).bind(&new_user.name).bind(&hashed_password).fetch_one(&mut **db).await
}

pub async fn login(
    mut db: Connection<AppDb>,
    login_params: &LoginParams,
) -> Result<String, String> {
    let user_res =
        sqlx::query_as::<_, User>(r#"SELECT * FROM rs_party.user as u WHERE email_address = $1;"#)
            .bind(&login_params.email_address)
            .fetch_one(&mut **db)
            .await;

    let pw_matched_usr = match user_res {
        Ok(user) => {
            let passwords_match_res = bcrypt::verify(&login_params.password, &user.password);

            match passwords_match_res {
                Ok(passwords_match) => match passwords_match {
                    true => Ok(user),
                    false => return Err("password mismatch".to_string()),
                },
                Err(e) => Err(e.to_string()),
            }
        }
        Err(error) => Err(error.to_string()),
    };

    let session_creation_result = match pw_matched_usr {
        Ok(user) => create_session(db, user).await,
        Err(err) => Err(err),
    };

    match session_creation_result {
        Ok(s) => Ok(s.session_key.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn log_request<'a>(
    mut db: Connection<AppDb>,
    req: &Request<'a>,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let time_received_opt = req.local_cache(|| crate::fairings::TimeStart(None)).0;

    let entry = RequestLogEntry {
        time_received: match time_received_opt {
            Some(time) => time,
            None => Utc::now(),
        },
        time_logged: Utc::now(),
        method: req.method().to_string(),
        req_url: req.uri().to_string(),
        req_headers: req
            .headers()
            .iter()
            .map(|h| format!("{}: {}", h.name(), h.value()))
            .collect::<Vec<String>>()
            .join(", "),
    };

    sqlx::query(
        r#"INSERT INTO rs_party.request_log (time_received, time_logged, method, req_url, req_headers) VALUES ($1, $2, $3, $4, $5);"#,
    ).bind(entry.time_received)
    .bind(entry.time_logged)
    .bind(entry.method)
    .bind(entry.req_url)
    .bind(entry.req_headers)
    .execute(&mut **db).await
}

pub async fn create_session(mut db: Connection<AppDb>, user: User) -> Result<Session, String> {
    let session = Session {
        session_key: Uuid::new_v4(),
        user_id: user.id,
        session_data: "".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    let query_result = sqlx::query_as::<_, Session>(
        r#"
INSERT INTO rs_party.session (session_key, user_id, session_data, created, updated)
VALUES ($1, $2, $3, $4, $5)
RETURNING *;
"#,
    )
    .bind(session.session_key)
    .bind(session.user_id)
    .bind(session.session_data)
    .bind(session.created)
    .bind(session.updated)
    .fetch_one(&mut **db)
    .await;

    match query_result {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}
