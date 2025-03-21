//! Data model for party planner application

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// A struct for representing users in memory
#[derive(Serialize, Default, Debug, FromRow)]
pub struct UserWithPassword {
    pub id: Option<i64>,
    pub email_address: String,
    pub name: String,
    pub password: Option<String>,
    pub is_superuser: bool,
    // TODO: Add created, updated, and last-logged-in times
}

/// Application user
#[derive(Serialize, Deserialize, Default, Debug, FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub email_address: String,
    pub name: String,
    pub is_superuser: bool,
}

impl From<SessionUser> for User {
    fn from(value: SessionUser) -> Self {
        User {
            id: Some(value.user_id),
            email_address: value.email_address,
            name: value.name,
            is_superuser: value.is_superuser,
        }
    }
}

impl From<UserWithPassword> for User {
    fn from(value: UserWithPassword) -> Self {
        User {
            id: value.id,
            email_address: value.email_address,
            name: value.name,
            is_superuser: value.is_superuser,
        }
    }
}

/// Data model for registration requests
#[derive(Deserialize, Debug, FromRow)]
pub struct NewUserParams {
    pub email: String,
    pub name: String,
    pub password: String,
}

/// Data model for login parameters
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

/// Entry into the db request log
#[derive(Debug)]
pub struct RequestLogEntry {
    pub id: Option<u64>,
    pub time_received: String,
    pub time_logged: String,
    pub method: String,
    pub req_url: String,
    pub req_headers: String,
}

/// Represents an authentication session
#[derive(Debug, FromRow)]
pub struct Session {
    pub session_key: uuid::Uuid,
    pub user_id: i64,
    pub session_data: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// Model for retrieving session and user data from database in one request
#[derive(Debug, FromRow)]
pub struct SessionUser {
    pub session_key: uuid::Uuid,
    pub user_id: i64,
    pub session_data: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub email_address: String,
    pub name: String,
    pub is_superuser: bool,
}

/// Event record
#[derive(FromRow, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Option<i64>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub place: String,
}

// https://stackoverflow.com/questions/76465657/how-do-i-create-custom-postgres-enum-types-in-rust-sqlx
// I was having a lot of trouble here because I wanted to define the role_type in postgres within the rs_party schema.
// It did not work and said the types were incompatible. However, when I refreshed the database having removed role_type
// from the schema, everything seemed to work just fine. This may be an issue with sqlx.
// TODO: Reproduce in controlled environment, write up coherent issue for sqlx
/// List of types of user role with respect to events
#[derive(Clone, Debug, sqlx::Type, Default, PartialEq)]
#[sqlx(type_name = "role_type", rename_all = "lowercase")]
pub enum RoleType {
    Owner,
    Organizer,
    #[default]
    Guest,
}

impl From<RoleType> for String {
    fn from(val: RoleType) -> Self {
        match val {
            RoleType::Owner => String::from("owner"),
            RoleType::Guest => String::from("guest"),
            RoleType::Organizer => String::from("organizer"),
        }
    }
}

/// Record for user roles with respect to events.
#[derive(FromRow, Default)]
pub struct Role {
    pub id: Option<i64>,
    pub role_type: RoleType,
    pub user_id: i64,
    pub event_id: i64,
}

/// Struct for representing errors which can be converted to API responses
#[derive(Debug, Clone, Default)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub message: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        (self.status_code, self.message.unwrap_or_default()).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ApiError {
                status_code: StatusCode::NOT_FOUND,
                ..Default::default()
            },
            sqlx::Error::ColumnNotFound(_) => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some("DB Error: Column not found".to_string()),
            },
            sqlx::Error::Database(e) => {
                ApiError::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
            _ => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some("Unhandled Database error".to_string()),
            },
        }
    }
}

impl From<StatusCode> for ApiError {
    fn from(status_code: StatusCode) -> Self {
        ApiError {
            status_code,
            message: None,
        }
    }
}

impl From<(StatusCode, String)> for ApiError {
    fn from((status_code, message): (StatusCode, String)) -> Self {
        ApiError {
            status_code,
            message: Some(message),
        }
    }
}

impl From<(StatusCode, &str)> for ApiError {
    fn from((status_code, message): (StatusCode, &str)) -> Self {
        ApiError {
            status_code,
            message: Some(message.to_string()),
        }
    }
}

impl ApiError {
    pub fn internal(msg: &str) -> ApiError {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: Some(msg.to_string()),
        }
    }
}
