use core::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Status {
    // Informational (1xx)
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    // Success (2xx)
    Ok = 200,
    Created = 201,

    // Redirection (3xx)
    MultipleChoices = 300,
    MovedPermanently = 301,

    // Client errors (4xx)
    BadRequest = 400,
    NotFound = 404,

    // Server errors (5xx)
    InternalServerError = 500,

    // Unimplemented (xxx)
    Unimplemented(usize) = 0,
}

impl Status {
    pub fn code(&self) -> usize {
        self.clone().into()
    }

    pub fn is_ok(&self) -> bool {
        let code = self.code();
        
        code >= 200 && code < 300
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.code(), match self {
            Self::BadRequest => "Bad Request",
            Self::Continue => "Continue",
            Self::Created => "Created",
            Self::EarlyHints => "Early Hints",
            Self::InternalServerError => "Internal Server Error",
            Self::MovedPermanently => "Moved Permanently",
            Self::MultipleChoices => "Multiple Choices",
            Self::NotFound => "Not Found",
            Self::Ok => "OK",
            Self::Processing => "Processing",
            Self::SwitchingProtocols => "Switching Protocols",
            _ => "Unimplemented",
        })
    }
}

impl From<usize> for Status {
    fn from(value: usize) -> Self {
        match value {
            100 => Self::Continue,
            101 => Self::SwitchingProtocols,
            102 => Self::Processing,
            103 => Self::EarlyHints,
            200 => Self::Ok,
            201 => Self::Created,
            300 => Self::MultipleChoices,
            301 => Self::MovedPermanently,
            400 => Self::BadRequest,
            404 => Self::NotFound,
            500 => Self::InternalServerError,
            code => Self::Unimplemented(code),
        }
    }
}

impl Into<usize> for Status {
    fn into(self) -> usize {
        match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,
            Self::Ok => 200,
            Self::Created => 201,
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
            Self::Unimplemented(code) => code,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseStatusError;

impl FromStr for Status {
    type Err = ParseStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code: usize = s.parse().or(Err(ParseStatusError))?;

        Ok(code.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        assert_eq!(Status::Ok.code(), 200);
    }

    #[test]
    fn test_is_ok() {
        assert!(!Status::Continue.is_ok());
        assert!(Status::Ok.is_ok());
        assert!(Status::Created.is_ok());
        assert!(!Status::MultipleChoices.is_ok());
        assert!(!Status::MovedPermanently.is_ok());
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Status::from_str("200"), Ok(Status::Ok));
    }
}