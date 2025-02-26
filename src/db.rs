extern crate bcrypt;
use chrono::Utc;
use sqlx::PgPool;

use sqlx;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::Postgres;
use uuid::Uuid;

use crate::model::{self, LoginParams, NewUserParams, RequestLogEntry, Session, User};

/// Create and return a database pool connection
pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/rs_party".to_string());

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_connection_str)
        .await
}

/// Get first user in the database. Probably "Admin"
pub async fn get_first_user(conn: &mut PoolConnection<Postgres>) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM rs_party.user LIMIT 1;"#)
        .fetch_one(&mut **conn)
        .await
}

/// Get all users from the database
pub async fn get_all_users(conn: &mut PoolConnection<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM rs_party.user"#)
        .fetch_all(&mut **conn)
        .await
}

/// Insert a new user into the database
pub async fn insert_user(
    mut conn: PoolConnection<Postgres>,
    new_user: &NewUserParams,
) -> Result<User, sqlx::Error> {
    // Create a secure hash with the password

    let hashed_password = match bcrypt::hash(&new_user.password, bcrypt::DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => "Garbage".to_string(),
    };

    sqlx::query_as::<_, User>(r#"INSERT INTO rs_party.user (email_address, name, password) VALUES ( $1, $2, $3 ) RETURNING id, email_address, name, is_superuser;"#
    ).bind(&new_user.email).bind(&new_user.name).bind(&hashed_password).fetch_one(&mut *conn).await
}

/// Take a set of login parameters; create new session; return session ID
pub async fn login(
    mut conn: PoolConnection<Postgres>,
    login_params: &LoginParams,
) -> Result<String, String> {
    let user_res =
        sqlx::query_as::<_, User>(r#"SELECT * FROM rs_party.user as u WHERE email_address = $1;"#)
            .bind(&login_params.email_address)
            .fetch_one(&mut *conn)
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
        Ok(user) => create_session(&mut conn, user).await,
        Err(err) => Err(err),
    };

    match session_creation_result {
        Ok(s) => Ok(s.session_key.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn create_session(
    conn: &mut PoolConnection<Postgres>,
    user: User,
) -> Result<Session, String> {
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
    .fetch_one(&mut **conn)
    .await;

    match query_result {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn insert_log_entry(
    logging_conn: &mut PoolConnection<Postgres>,
    entry: RequestLogEntry,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
INSERT INTO rs_party.request_log (time_received, time_logged, method, req_url, req_headers)
VALUES ($1, $2, $3, $4, $5);
"#,
    )
    .bind(entry.time_received)
    .bind(entry.time_logged)
    .bind(entry.req_url)
    .bind(entry.req_headers)
    .execute(&mut **logging_conn)
    .await
}

pub async fn insert_event(
    conn: &mut PoolConnection<Postgres>,
    new_event: &model::Event,
) -> Result<model::Event, sqlx::Error> {
    sqlx::query_as::<_, model::Event>(
        r#"
  INSERT INTO rs_party.event
  (start_date, end_date, start_time, end_time, place)
  VALUES ($1, $2, $3, $4, $5)
  RETURNING *;
  "#,
    )
    .bind(new_event.start_date)
    .bind(new_event.end_date)
    .bind(new_event.start_time)
    .bind(new_event.end_time)
    .bind(new_event.place.clone())
    .fetch_one(&mut **conn)
    .await
}

pub async fn get_event(
    conn: &mut PoolConnection<Postgres>,
    event_id: &i64,
) -> Result<model::Event, sqlx::Error> {
    sqlx::query_as::<_, model::Event>(r#" SELECT * FROM rs_party.event e WHERE e.id = $1;"#)
        .bind(event_id)
        .fetch_one(&mut **conn)
        .await
}

pub async fn update_event(
    conn: &mut PoolConnection<Postgres>,
    event: &model::Event,
) -> Result<model::Event, sqlx::Error> {
    sqlx::query_as::<_, model::Event>(
        r#"
  UPDATE rs_party.event SET (start_date, end_date, start_time, end_time, place) 
  = ($1, $2, $3, $4, $5)
  WHERE id = $6
  RETURNING *;
  "#,
    )
    .bind(event.start_date)
    .bind(event.end_date)
    .bind(event.start_time)
    .bind(event.end_time)
    .bind(event.place.clone())
    .bind(event.id)
    .fetch_one(&mut **conn)
    .await
}

pub async fn delete_event(
    conn: &mut PoolConnection<Postgres>,
    event_id: &i64,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(r#"DELETE FROM rs_party.event e WHERE e.id = $1;"#)
        .bind(event_id)
        .execute(&mut **conn)
        .await
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    use super::*;

    #[tokio::test]
    async fn test_event_crud() {
        let pool = get_pool().await.expect("Couldn't get db pool");
        let mut conn = pool.acquire().await.expect("Couldn't get db connection");

        let e = model::Event {
            id: None,
            start_date: NaiveDate::from_ymd_opt(2025, 02, 25).expect("bad date"),
            end_date: NaiveDate::from_ymd_opt(2025, 02, 25).expect("bad date"),
            start_time: None,
            end_time: None,
            place: "Somewhere good".to_string(),
        };

        let mut out_e = insert_event(&mut conn, &e)
            .await
            .expect("could not insert event");

        assert_eq!(e.start_date, out_e.start_date);
        assert_ne!(out_e.id, None);

        out_e.place += " - no, someplace better!";

        let updated_e = update_event(&mut conn, &out_e).await.expect("Failed to update event");

        assert_eq!(updated_e.id, out_e.id);
        assert_eq!(updated_e.place, out_e.place);
        assert_ne!(updated_e.place, e.place);

        let event_id = out_e.id.expect("no output ID");
        let _delete_result = delete_event(&mut conn, &event_id).await;
        let get_result = get_event(&mut conn, &event_id).await;

        match get_result {
          Ok(_) => assert!(false),
          Err(err) => println!("{}", err)
        }
    }

}
