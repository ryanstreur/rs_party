use rocket::fairing::{Fairing, Info, Kind};

pub struct RequestLoggerFairing;

impl Fairing for RequestLoggerFairing {
  fn info(&self) -> Info {
    Info {
      name: "RequestLogger",
      kind: Kind::Response
    }
  }
}
