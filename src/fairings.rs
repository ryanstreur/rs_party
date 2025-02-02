use std::borrow::Cow;

use chrono::{DateTime, Utc};

use rocket::{self, Build, Orbit, Rocket};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uncased::Uncased, Header},
    Data, Request, Response,
};
use rocket_db_pools::Database;


use crate::db::AppDb;

pub struct TimingHeader {
    time_received: DateTime<Utc>,
}

pub struct TimeStart(pub Option<DateTime<Utc>>);

impl<'h> Into<Header<'h>> for TimingHeader {
    fn into(self) -> Header<'h> {
        Header {
            name: Uncased::from("time_received"),
            value: Cow::from(self.time_received.to_string()),
        }
    }
}

pub struct RequestLoggerFairing<'a> {
  rocket: &'a Rocket<Orbit>
}

#[rocket::async_trait]
impl Fairing for RequestLoggerFairing {
    fn info(&self) -> Info {
        Info {
            name: "RequestLogger",
            kind: Kind::Liftoff | Kind::Request | Kind::Response,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
      self.rocket = rocket;
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let timing_header = TimingHeader {
            time_received: Utc::now(),
        };

        req.add_header(timing_header);
        req.local_cache(|| TimeStart(Some(Utc::now())));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
      let mut time_received_headers = req.headers().get("time_received");

      // Local cache technique
      let start_time_cached = req.local_cache(|| TimeStart(None));
      match start_time_cached.0 {
        Some(start_time) => {
          println!("Start time from cache: {}", start_time.to_string());
          let end_time = Utc::now();

          println!("End time from cache: {}", end_time.to_string());

          let millis = end_time.timestamp_millis() - start_time.timestamp_millis();
          println!("Milliseconds for request: {}", millis);

          res.set_raw_header("X-Response-Time", format!("{} ms", millis));
        },
        None => println!("Failed to retrieve start time from cache")
      }

      let db = AppDb::fetch(self.rocket).unwrap();

      

      // Header technique
      let time_received_option = time_received_headers.next();
      match time_received_option {
        Some(time_str) => println!("Time received: {}", time_str),
        None => println!("Time Received not stored!")
      }
    }
}
