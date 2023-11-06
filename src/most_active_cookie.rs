use std::{
    collections::{HashMap, HashSet},
    error::Error as StdError,
    str::FromStr,
};

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

fn max_occurence_cookies(data: &str, date: Date) -> Result<HashSet<Cookie>, Error> {
    let Log(data) = data.parse()?;

    let mut occurrences: HashMap<Cookie, usize> = HashMap::new();

    for entry in data {
        if entry.timestamp.date == date {
            *occurrences.entry(entry.cookie).or_insert(0) += 1;
        }
    }

    let max_occurrences = occurrences.values().max().cloned().unwrap_or(0);

    Ok(occurrences
        .iter()
        .filter(|(_, count)| **count == max_occurrences)
        .map(|(a, _)| a)
        .cloned()
        .collect())
}

#[cfg(test)]
mod tests {
    use quantcast_exercise::{Cookie, Date};

    use crate::max_occurence_cookies;

    #[test]
    fn integration() {
        assert_eq!(
            max_occurence_cookies(
                r#"cookie,timestamp
AtY0laUfhglK3lC7,2018-12-09T14:19:00+00:00
SAZuXPGUrfbcn5UA,2018-12-09T10:13:00+00:00
5UAVanZf6UtGyKVS,2018-12-09T07:25:00+00:00
AtY0laUfhglK3lC7,2018-12-09T06:19:00+00:00
SAZuXPGUrfbcn5UA,2018-12-08T22:03:00+00:00
4sMM2LxV07bPJzwf,2018-12-08T21:30:00+00:00
fbcn5UAVanZf6UtG,2018-12-08T09:30:00+00:00
4sMM2LxV07bPJzwf,2018-12-07T23:30:00+00:00"#,
                Date {
                    year: 2018,
                    month: 12,
                    day: 8
                }
            )
            .unwrap(),
            vec!["SAZuXPGUrfbcn5UA", "4sMM2LxV07bPJzwf", "fbcn5UAVanZf6UtG"]
                .into_iter()
                .map(|a| Cookie(a.to_owned()))
                .collect()
        );
    }
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let data = std::fs::read_to_string(args.path)?;

    let cookies = max_occurence_cookies(&data, args.date)?;

    println!(
        "{}",
        cookies
            .into_iter()
            .map(|a| a.0)
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(())
}
