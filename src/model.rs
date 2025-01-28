//! Data model for party planner application

use rocket::{data::FromData, serde::{Deserialize, Serialize, }};
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
#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct User {
  pub id: usize,
  pub email: String,
  pub name: String,
  pub is_superuser: bool,
  // TODO: Add created, updated, and last-logged-in times
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUserParams {
  pub email: String,
  pub name: String,
  pub password: String
}

struct PartyUser {
  id: usize
}
