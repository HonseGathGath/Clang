use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

// 1. Finish the definition
#[derive(Debug, PartialEq, Eq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

// 2. Implement the `Error` trait
//
//
impl Display for ParsePercentageError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParsePercentageError::OutOfRange => write!(f, "OutOfRange"),
            ParsePercentageError::InvalidInput => write!(f, "InvalidInput"),
        }
    }
}

impl Error for ParsePercentageError {
    fn description(&self) -> &str {
        match self {
            ParsePercentageError::InvalidInput => "Input can not be parsed",
            ParsePercentageError::OutOfRange => "Input out of range",
        }
    }
}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    let percentage: u8 = input
        .parse()
        .map_err(|_| ParsePercentageError::InvalidInput)?;

    if percentage <= 100 {
        Ok(percentage)
    } else {
        Err(ParsePercentageError::OutOfRange)
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
