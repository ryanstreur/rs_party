extern crate bcrypt;
use axum::http::StatusCode;
use chrono::Utc;
use sqlx::PgPool;

use sqlx;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::Postgres;
use uuid::Uuid;

use crate::conf::get_db_connection_string;
use crate::model::{
    self, ApiError, LoginParams, NewUserParams, RequestLogEntry, Session, SessionUser,
    UserWithPassword,
};

/// Create and return a database pool connection
pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let db_conn_string = get_db_connection_string();

    println!(
        "Attempting pool connection with connection string: {}",
        db_conn_string
    );

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_conn_string)
        .await
}

/// Get first user in the database. Probably "Admin"
pub async fn get_first_user(
    conn: &mut PoolConnection<Postgres>,
) -> Result<UserWithPassword, sqlx::Error> {
    sqlx::query_as::<_, UserWithPassword>(r#"SELECT * FROM rs_party.user LIMIT 1;"#)
        .fetch_one(&mut **conn)
        .await
}

/// Get all users from the database
pub async fn get_all_users(
    conn: &mut PoolConnection<Postgres>,
) -> Result<Vec<UserWithPassword>, sqlx::Error> {
    sqlx::query_as::<_, UserWithPassword>(r#"SELECT * FROM rs_party.user"#)
        .fetch_all(&mut **conn)
        .await
}

/// Insert a new user into the database
pub async fn insert_user(
    conn: &mut PoolConnection<Postgres>,
    new_user: &NewUserParams,
) -> Result<UserWithPassword, sqlx::Error> {
    // Create a secure hash with the password

    let hashed_password = match bcrypt::hash(&new_user.password, bcrypt::DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => "Garbage".to_string(),
    };

    sqlx::query_as::<_, UserWithPassword>(
        r#"
    INSERT INTO rs_party.user
    (email_address, name, password)
    VALUES ( $1, $2, $3 )
    RETURNING id, email_address, name, is_superuser, password;
    "#,
    )
    .bind(&new_user.email)
    .bind(&new_user.name)
    .bind(&hashed_password)
    .fetch_one(&mut **conn)
    .await
}

pub async fn get_user(
    conn: &mut PoolConnection<Postgres>,
    user_id: &i64,
) -> Result<model::UserWithPassword, sqlx::Error> {
    sqlx::query_as::<_, model::UserWithPassword>(
        r#"
    SELECT id, name, email_address, is_superuser, email_confirmed, password
    FROM rs_party.user u 
    WHERE u.id = $1;
    "#,
    )
    .bind(user_id)
    .fetch_one(&mut **conn)
    .await
}

pub async fn update_user(
    conn: &mut PoolConnection<Postgres>,
    user: &model::UserWithPassword,
) -> Result<model::UserWithPassword, sqlx::Error> {
    sqlx::query_as::<_, model::UserWithPassword>(
        r#"
  UPDATE rs_party.user SET 
  (name, email_address, password, is_superuser)
  VALUES ($1, $2, $3, $4)
  WHERE id = $5
  RETURNING *;
  "#,
    )
    .bind(&user.name)
    .bind(&user.email_address)
    .bind(&user.password)
    .bind(user.is_superuser)
    .fetch_one(&mut **conn)
    .await
}

pub async fn delete_user(
    conn: &mut PoolConnection<Postgres>,
    user_id: &i64,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(r#"DELETE FROM rs_party.user u WHERE u.id = $1;"#)
        .bind(user_id)
        .execute(&mut **conn)
        .await
}

/// Take a set of login parameters; create new session; return session ID
pub async fn login(
    mut conn: PoolConnection<Postgres>,
    login_params: &LoginParams,
) -> Result<String, ApiError> {
    let user_res = sqlx::query_as::<_, UserWithPassword>(
        r#"SELECT * FROM rs_party.user as u WHERE email_address = $1;"#,
    )
    .bind(&login_params.email)
    .fetch_one(&mut *conn)
    .await;

    let user = match user_res {
        Ok(u) => u,
        Err(e) => return Err(ApiError::from(e)),
    };

    let password = match &user.password {
        Some(p) => p,
        None => return Err(ApiError::internal("User has no password")),
    };

    let passwords_match_res = bcrypt::verify(&login_params.password, password);

    let pw_matched_user = match passwords_match_res {
        Ok(passwords_match) => match passwords_match {
            true => Ok(user),
            false => return Err(ApiError::internal("password mismatch")),
        },
        Err(e) => {
            return Err(ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(e.to_string()),
            })
        }
    };

    let session_creation_result = match pw_matched_user {
        Ok(user) => create_session(&mut conn, &user).await,
        Err(err) => return Err(ApiError::internal(err)),
    };

    match session_creation_result {
        Ok(s) => Ok(s.session_key.to_string()),
        Err(e) => Err(e),
    }
}

/// Create new session in database
pub async fn create_session(
    conn: &mut PoolConnection<Postgres>,
    user: &UserWithPassword,
) -> Result<Session, ApiError> {
    let user_id = match user.id {
        Some(id) => id,
        None => {
            return Err(ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some("Tried to create session using id-less user".to_string()),
            })
        }
    };

    let session = Session {
        session_key: Uuid::new_v4(),
        user_id,
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
        Err(e) => Err(ApiError::from(e)),
    }
}

/// Delete a session in the database by id
pub async fn delete_session(
    conn: &mut PoolConnection<Postgres>,
    session_key: &Uuid,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
DELETE FROM rs_party.session s WHERE s.session_key = $1"#,
    )
    .bind(session_key)
    .execute(&mut **conn)
    .await
}

pub async fn get_user_from_session_key(
    conn: &mut PoolConnection<Postgres>,
    session_key: &Uuid,
) -> Result<SessionUser, sqlx::Error> {
    sqlx::query_as::<_, SessionUser>(
        r#"
select 
    s.session_key, 
    u.id as user_id,
    s.session_data,
    s.created,
    s.updated,
    u.email_address,
    u.name,
    u.is_superuser
from rs_party.user u
    join rs_party.session s on u.id = s.user_id
where s.session_key = $1
order by s.created desc, s.updated desc
limit 1;
    "#,
    )
    .bind(session_key)
    .fetch_one(&mut **conn)
    .await
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
    let result = sqlx::query_as::<_, model::Event>(
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
    .await;

    match result {
        Ok(r) => Ok(r),
        Err(e) => {
            tracing::error!("Database error: {}", e.to_string());
            Err(e)
        }
    }
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

/// Takes a user ID and returns the events that user owns
pub async fn owned_events(
    conn: &mut PoolConnection<Postgres>,
    user_id: &i64,
) -> Result<Vec<model::Event>, sqlx::Error> {
    sqlx::query_as::<_, model::Event>(
        r#"
select
	e.*
from rs_party.user u
	join rs_party."role" r on r.user_id = u.id
	join rs_party."event" e on r.event_id = e.id
where r."role_type" = 'owner' and u.id = $1;
"#,
    )
    .bind(user_id)
    .fetch_all(&mut **conn)
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

pub async fn insert_role(
    conn: &mut PoolConnection<Postgres>,
    new_role: &model::Role,
) -> Result<model::Role, sqlx::Error> {
    sqlx::query_as::<_, model::Role>(
        r#"
  INSERT INTO rs_party.role
  (role_type, user_id, event_id)
  VALUES ($1, $2, $3)
  RETURNING *;
  "#,
    )
    .bind(new_role.role_type.clone())
    .bind(new_role.user_id)
    .bind(new_role.event_id)
    .fetch_one(&mut **conn)
    .await
}

pub async fn get_role(
    conn: &mut PoolConnection<Postgres>,
    role_id: &i64,
) -> Result<model::Role, sqlx::Error> {
    sqlx::query_as::<_, model::Role>(r#" SELECT * FROM rs_party.role e WHERE e.id = $1;"#)
        .bind(role_id)
        .fetch_one(&mut **conn)
        .await
}

pub async fn update_role(
    conn: &mut PoolConnection<Postgres>,
    role: &model::Role,
) -> Result<model::Role, sqlx::Error> {
    sqlx::query_as::<_, model::Role>(
        r#"
  UPDATE rs_party.role SET 
  (role_type, user_id, event_id)
  VALUES ($1, $2, $3)

  WHERE id = $6
  RETURNING *;
  "#,
    )
    .bind(role.role_type.clone())
    .bind(role.user_id)
    .bind(role.event_id)
    .bind(role.id)
    .fetch_one(&mut **conn)
    .await
}

pub async fn delete_role(
    conn: &mut PoolConnection<Postgres>,
    role_id: &i64,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(r#"DELETE FROM rs_party.role e WHERE e.id = $1;"#)
        .bind(role_id)
        .execute(&mut **conn)
        .await
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    use super::*;

    /// Test create/read/update/delete for events
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

        let updated_e = update_event(&mut conn, &out_e)
            .await
            .expect("Failed to update event");

        assert_eq!(updated_e.id, out_e.id);
        assert_eq!(updated_e.place, out_e.place);
        assert_ne!(updated_e.place, e.place);

        let event_id = out_e.id.expect("no output ID");
        let _delete_result = delete_event(&mut conn, &event_id).await;
        let get_result = get_event(&mut conn, &event_id).await;

        match get_result {
            Ok(_) => assert!(false),
            Err(err) => println!("{}", err),
        }
    }

    #[tokio::test]
    pub async fn test_role_crud() {
        let pool = get_pool().await.expect("Couldn't get db pool");
        let mut conn = pool.acquire().await.expect("Couldn't get db connection");

        let u = NewUserParams {
            name: "Test User".to_string(),
            password: "password not yet hashed".to_string(),
            email: "test@example.com".to_string(),
        };

        let saved_user = insert_user(&mut conn, &u)
            .await
            .expect("couldn't insert user");

        // 2/25/2025 spent all night tonight trying to figure out why I was getting a "column not found" error from sqlx. I thought (without evidence) that if I just made something an Option type in a struct which derived "FromRow" that it would gracefully handle result sets which did not contain a column corresponding to the Optioned field. Boy was I wrong.

        let user_id = saved_user.id.expect("saved user should have an ID");

        let new_event = model::Event {
            ..Default::default()
        };

        let saved_event = insert_event(&mut conn, &new_event)
            .await
            .expect("failed to insert event");

        let event_id = saved_event.id.expect("Saved event should have an ID");

        let new_role = model::Role {
            user_id,
            role_type: model::RoleType::Owner,
            event_id,
            ..Default::default()
        };

        let saved_role = insert_role(&mut conn, &new_role)
            .await
            .expect("Role insertion failed");

        let role_id = saved_role.id.expect("Role saved with no ID");

        // Clean up
        // Have to delete in this order because of foreign key constraints
        let _role_deletion = delete_role(&mut conn, &role_id)
            .await
            .expect("role deletion failed");

        let _event_deletion = delete_event(&mut conn, &event_id)
            .await
            .expect("event deletion failed");

        let _user_deletion = delete_user(&mut conn, &user_id)
            .await
            .expect("should be able to delete");
    }

    #[tokio::test]
    pub async fn test_user_from_session() {
        let pool = get_pool().await.expect("Couldn't get db pool");
        let mut conn = pool.acquire().await.expect("Couldn't get db connection");

        let u = NewUserParams {
            name: "Test User".to_string(),
            password: "password not yet hashed".to_string(),
            email: "test@example.com".to_string(),
        };

        let saved_user = insert_user(&mut conn, &u)
            .await
            .expect("couldn't insert user");

        let session = create_session(&mut conn, &saved_user)
            .await
            .expect("Couldn't create session");

        let user_id = saved_user.id.expect("No ID on saved user");

        let second_user_result = get_user_from_session_key(&mut conn, &session.session_key).await;

        let second_user = match second_user_result {
            Ok(u) => u,
            Err(e) => {
                assert_eq!(e.to_string(), "".to_string());
                panic!("Should be able to get user");
            }
        };

        let second_user_id = second_user.user_id;

        assert_eq!(user_id, second_user_id);

        let _session_deletion = delete_session(&mut conn, &session.session_key)
            .await
            .expect("Couldn't delete session");

        let _user_deletion = delete_user(&mut conn, &user_id)
            .await
            .expect("Couldn't delete user");
    }
}
