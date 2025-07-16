use std::{collections::HashMap, fmt::Display, str::FromStr};

const GEN_DELIMS: [char; 7] = [':', '/', '?', '#', '[', ']', '@'];
const SUB_GEN_DELIMS: [char; 11] = ['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '='];

fn is_char_unreserved(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ['-', '.', '_', '~'].contains(&ch)
}

#[derive(Clone, Debug)]
pub struct Uri {
    pub path: String,
    pub scheme: String,
    pub search_params: HashMap<String, String>,
}

impl Uri {
    pub fn authority(&self) -> Option<String> {
        Some(String::new())
    }
}

pub enum ParseUriError {
    InvalidScheme,
    SuddenEol,
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.authority() {
            Some(authority) => write!(f, "{}://{}{}", self.scheme, authority, self.path),
            None => write!(f, "{}:{}", self.scheme, self.path),
        }
    }
}

impl FromStr for Uri {
    type Err = ParseUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_index = 0;
        let mut chars = s.chars().collect::<Vec<char>>();

        // Parse scheme

        let mut scheme = String::new();

        loop {
            let ch = chars.get(char_index).ok_or(ParseUriError::SuddenEol)?;
            char_index += 1;

            if *ch == ':' {
                break;
            }

            if !ch.is_ascii_alphanumeric() {
                return Err(ParseUriError::InvalidScheme);
            }

            if !['+', '.', '-'].contains(&ch) {
                return Err(ParseUriError::InvalidScheme);
            }

            scheme.push(*ch);
        }

        // Parse authority


        let mut authority = None;

        // Authorities start with `//` so we need to do a lookahead
        if chars.get(char_index) == Some(&'/') && chars.get(char_index + 1) == Some(&'/') {
            char_index += 2;

            // Parse scheme

            let mut scheme_string = String::new();

            loop {
                let ch = chars.get(char_index).ok_or(ParseUriError::SuddenEol)?;

                if *ch == ':' {
                    break;
                }

                if !ch.is_ascii_alphanumeric() {
                    return Err(ParseUriError::InvalidScheme);
                }

                if !['+', '.', '-'].contains(&ch) {
                    return Err(ParseUriError::InvalidScheme);
                }

                scheme_string.push(*ch);

                char_index += 1;
            }

            authority = Some(scheme_string);
        }

        // Parse path

        let mut path = String::new();

        loop {}

        // Parse search params

        let mut search_params = HashMap::new();

        Ok(Uri {
            scheme,
            path,
            search_params,
        })
    }
}