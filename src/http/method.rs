use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMethodError;

impl FromStr for Method {
    type Err = ParseMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            _ => Err(ParseMethodError),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::GET => "GET",
            Self::POST => "POST",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(Method::from_str("GET"), Ok(Method::GET));
        assert_eq!(Method::from_str("POST"), Ok(Method::POST));
        assert_eq!(Method::from_str("Invalid"), Err(ParseMethodError));
    }
}