pub mod date;
use std::str::FromStr;

pub use date::{Date, Timestamp};

mod csv;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Cookie(pub String);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum CookieError {}

impl FromStr for Cookie {
    type Err = CookieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cookie(s.to_owned()))
    }
}

csv_parser!(
    pub Log, LogEntry, LogParseError
    cookie, CookieError,
        "cookie"    : Cookie,
    timestamp, TimestampError,
        "timestamp" : crate::Timestamp
);
