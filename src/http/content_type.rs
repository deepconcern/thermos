use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum ApplicationType {
    #[default]
    OctetStream,
}

impl Display for ApplicationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::OctetStream => "octet-stream",
        })
    }
}

#[derive(Clone, Debug)]
pub struct ParseApplicationTypeError;

impl FromStr for ApplicationType {
    type Err = ParseApplicationTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "octet-stream" => Ok(ApplicationType::OctetStream),
            _ => Err(ParseApplicationTypeError),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextType {
    #[default]
    Plain,
}

impl Display for TextType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Plain => "plain",
        })
    }
}

#[derive(Clone, Debug)]
pub struct ParseTextTypeError;

impl FromStr for TextType {
    type Err = ParseTextTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(TextType::Plain),
            _ => Err(ParseTextTypeError),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ContentType {
    Application(ApplicationType),
    Text(TextType),
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Application(ApplicationType::default())
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Application(application_type) => {
                write!(f, "application/{}", application_type)
            },
            Self::Text(text_type) => {
                write!(f, "text/{}", text_type)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParseContentTypeError;

impl FromStr for ContentType {
    type Err = ParseContentTypeError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Err(ParseContentTypeError)
    }
}
