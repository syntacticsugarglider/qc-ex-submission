use std::{error::Error as StdError, str::FromStr, collections::HashMap};

use quantcast_exercise::{Cookie, Date, Log, LogParseError};

use clap::Parser;

fn parse_date(s: &str) -> Result<Date, Box<dyn StdError + Send + Sync + 'static>> {
    <Date as FromStr>::from_str(s)
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync + 'static>)
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Csv(LogParseError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<LogParseError> for Error {
    fn from(e: LogParseError) -> Self {
        Error::Csv(e)
    }
}

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    #[arg(short, value_parser = parse_date)]
    date: Date,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let data = std::fs::read_to_string(args.path)?;

    let Log(data) = data.parse()?;

    let mut occurrences: HashMap<Cookie, usize> = HashMap::new();

    for entry in data {
        if entry.timestamp.date == args.date {
            *occurrences.entry(entry.cookie).or_insert(0) += 1;
        }
    }

    let max_occurrences = occurrences.values().max().cloned().unwrap_or(0);

    for (cookie, _) in occurrences.iter().filter(|(_, count)| **count == max_occurrences) {
        println!("{}", cookie.0);
    }

    Ok(())
}
