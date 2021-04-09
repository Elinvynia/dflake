//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! # dflake
//!
//! A simple Discord snowflake parsing library. Comes with optional chrono support using the `chrono_support` feature.
//!
//! ## Sample Usage
//! ```rust
//! let dflake = dflake::parse(3971046231244935168);
//! println!("My internal process ID is: {}", dflake.process_id);
//! ```
//!
//! [ci]: https://github.com/Elinvynia/dflake/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/dflake/Rust/master?style=flat-square
//! [docs]: https://docs.rs/dflake
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/dflake
//! [crate-version]: https://img.shields.io/crates/v/dflake.svg?style=flat-square

#![forbid(unsafe_code)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::fmt;
use std::num::ParseIntError;

#[cfg(feature = "chrono_support")]
use chrono::NaiveDateTime;

/// Structure holding a valid Discord snowflake.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dflake {
    raw: u64,
    /// Unix timestamp in miliseconds, represented as an u32.
    pub timestamp: u32,
    /// Internal worker ID.
    pub worker_id: u8,
    /// Internal process ID.
    pub process_id: u8,
    /// The amount of times an ID was created on a process.
    pub increment: u16,
}

impl Dflake {
    /// Returns the raw u64 this Dflake was made from.
    pub fn raw(&self) -> u64 {
        self.raw
    }

    #[cfg(feature = "chrono_support")]
    /// Returns the timestamp as a chrono::NaiveDateTime
    pub fn datetime(&self) -> NaiveDateTime {
        let seconds = self.timestamp / 1000;
        NaiveDateTime::from_timestamp(seconds as i64, 0)
    }
}

/// Tries to parse a u64 into a [`Dflake`].
pub fn parse(input: u64) -> Dflake {
    Dflake {
        raw: input,
        timestamp: ((input >> 22) + 1420070400000) as u32,
        worker_id: ((input & 0x3E0000) >> 17) as u8,
        process_id: ((input & 0x1F000) >> 12) as u8,
        increment: (input & 0xFFF) as u16,
    }
}

/// Tries to parse a string into a [`Dflake`].
pub fn parse_str<T: AsRef<str>>(input: T) -> Result<Dflake, ParseError> {
    let input = input.as_ref();

    if input.chars().any(|ch| ch.is_whitespace()) {
        return Err(ParseError::ContainsWhitespace);
    }

    if input.chars().any(|ch| !ch.is_numeric()) {
        return Err(ParseError::InvalidChar);
    }

    if input.len() > 20 {
        return Err(ParseError::TooLarge);
    }

    let num: u64 = input.parse()?;
    Ok(parse(num))
}

/// Things that can go wrong while parsing a [`Dflake`].
#[derive(Debug)]
pub enum ParseError {
    /// The provided string or number was too small.
    TooSmall,
    /// The provided string or number was too large.
    TooLarge,
    /// The string contains whitespace.
    ContainsWhitespace,
    /// The string contains a non-numeric character.
    InvalidChar,
    /// Failed to parse the string into a number.
    ParseIntError(ParseIntError),
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseError::*;
        let message = match self {
            TooSmall => "The input is too small".into(),
            TooLarge => "The input is too large.".into(),
            ContainsWhitespace => "The input contains whitespace.".into(),
            InvalidChar => "The input contains an invalid character".into(),
            ParseIntError(e) => format!("Failed to parse as an integer: {}", e),
        };

        write!(f, "{}", message)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::ParseIntError(e)
    }
}
