use std::{error::Error, fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat,
    InvalidLiteral(ParseIntError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat => {
                write!(f, "invalid format")
            }
            ParseError::InvalidLiteral(e) => {
                write!(f, "invalid literal: {}", e)
            }
        }
    }
}

impl Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::InvalidLiteral(e)
    }
}

impl FromStr for Date {
    type Err = ParseError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        fn parse_segment<T: FromStr<Err = ParseIntError>>(
            data: &str,
        ) -> Result<(T, &str), ParseError> {
            data.split_once('-')
                .ok_or(ParseError::InvalidFormat)
                .and_then(|(segment, rest)| Ok((segment.parse()?, rest)))
        }

        let (year, data) = parse_segment(data)?;
        let (month, data) = parse_segment(data)?;
        let day = data.parse()?;

        Ok(Date { month, day, year })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timestamp {
    pub date: Date,
    pub time: String,
}

impl FromStr for Timestamp {
    type Err = <Date as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, time) = s.split_once('T').unwrap_or((s, ""));
        Ok(Timestamp {
            date: s.parse()?,
            time: time.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Date, Timestamp, ParseError};

    #[test]
    fn invalid_format() {
        assert_eq!(
            <Date as FromStr>::from_str(""),
            Err(ParseError::InvalidFormat)
        );

        assert_eq!(
            <Date as FromStr>::from_str("1926"),
            Err(ParseError::InvalidFormat)
        );

        assert_eq!(
            <Date as FromStr>::from_str("1926-121"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn invalid_literal() {
        assert!(matches!(
            <Date as FromStr>::from_str("1926-12-1-12"),
            Err(ParseError::InvalidLiteral(_))
        ));

        assert!(matches!(
            <Date as FromStr>::from_str("----"),
            Err(ParseError::InvalidLiteral(_))
        ));

        assert!(matches!(
            <Date as FromStr>::from_str("A-A-A-A"),
            Err(ParseError::InvalidLiteral(_))
        ));
    }

    #[test]
    fn valid_date() {
        assert_eq!(
            <Date as FromStr>::from_str("1926-12-1"),
            Ok(Date {
                year: 1926,
                month: 12,
                day: 1
            })
        );

        assert_eq!(
            <Date as FromStr>::from_str("25-1-32"),
            Ok(Date {
                year: 25,
                month: 1,
                day: 32
            })
        );
    }

    #[test]
    fn valid_timestamp() {
        assert_eq!(
            <Timestamp as FromStr>::from_str("2018-12-09T14:19:00+00:00"),
            Ok(Timestamp {
                date: Date {
                    year: 2018,
                    month: 12,
                    day: 9
                },
                time: "14:19:00+00:00".to_owned()
            })
        );
    }
}