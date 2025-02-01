//! Data model for party planner application

use rocket::serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

enum PartyRole {
  Guest,
  Organizer,
  Owner,
}

enum Rsvp {
  Yes,
  No,
  Maybe,
  Pending
}

/// A struct for representing users in memory
#[derive(Serialize, Default, Debug, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
  pub id: i64,
  pub email_address: String,
  pub name: String,
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

