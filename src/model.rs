//! Data model for party planner application

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

// enum PartyRole {
//   Guest,
//   Organizer,
//   Owner,
// }

// enum Rsvp {
//   Yes,
//   No,
//   Maybe,
//   Pending
// }

/// A struct for representing users in memory
#[derive(Serialize, Default, Debug, FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub email_address: String,
    pub name: String,
    pub password: Option<String>,
    pub is_superuser: bool,
    // TODO: Add created, updated, and last-logged-in times
}

#[derive(Deserialize, Debug, FromRow)]
pub struct NewUserParams {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginParams {
    pub email_address: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {}

#[derive(Debug)]
pub struct RequestLogEntry {
    pub id: Option<u64>,
    pub time_received: String,
    pub time_logged: String,
    pub method: String,
    pub req_url: String,
    pub req_headers: String,
}

#[derive(Debug, FromRow)]
pub struct Session {
    pub session_key: uuid::Uuid,
    pub user_id: i64,
    pub session_data: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct Event {
    pub id: Option<i64>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub place: String,
}

#[derive(FromRow)]
pub struct Role {
    pub id: Option<i64>,
    pub role_type: String,
    pub user_id: i64,
    pub event_id: i64,
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status_code: i32,
    pub error: String,
}
