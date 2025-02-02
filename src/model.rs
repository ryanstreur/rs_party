//! Data model for party planner application

use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
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
#[serde(crate = "rocket::serde")]
pub struct User {
  pub id: i64,
  pub email_address: String,
  pub name: String,
  pub password: String,
  pub is_superuser: bool,
  // TODO: Add created, updated, and last-logged-in times
}

#[derive(Deserialize, Debug, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct NewUserParams {
  pub email: String,
  pub name: String,
  pub password: String
}


#[derive(Serialize, rocket::serde::Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginParams {
  pub email_address: String,
  pub password: String
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
  
}

#[derive(Debug)]
pub struct RequestLogEntry {
    pub time_received: DateTime<Utc>,
    pub time_logged: DateTime<Utc>,
    pub method: String,
    pub req_url: String,
    pub req_headers: String
}
