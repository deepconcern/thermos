use std::{
    collections::HashMap,
    io::{self, Bytes, Read},
    str::{from_utf8, FromStr, Utf8Error},
};

use crate::http::{content_type::ContentType, method::Method, uri::Uri};

const CR_BYTE: u8 = b'\r';
const LF_BYTE: u8 = b'\n';

fn clean_header(header: &str) -> String {
    let mut cleaned_header = String::with_capacity(header.len());
    let mut is_word_beginning = true;

    for c in header.chars() {
        if c == '-' {
            is_word_beginning = true;
        }

        if c.is_alphabetic() && is_word_beginning {
            cleaned_header.push(c.to_ascii_uppercase());
            is_word_beginning = false;
        } else {
            cleaned_header.push(c.to_ascii_lowercase());
        }
    }

    cleaned_header
}

#[derive(Clone, Debug)]
pub struct Request {
    pub content_type: ContentType,
    headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub method: Method,
    pub uri: Uri,
    pub version: String,
}

#[derive(Debug)]
pub enum RequestReadError {
    EmptyRequest,
    InvalidContentLength((usize, usize)),
    InvalidMethod(String),
    InvalidUri(String),
    IoError(io::Error),
    MalformedHeader(String),
    MalformedRequest(String),
}

impl From<io::Error> for RequestReadError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

fn read_line<R>(bytes: &mut Bytes<R>) -> Result<String, io::Error>
where
    R: Read,
{
    let mut buf = Vec::new();

    let mut was_cr = false;

    while let Some(read_result) = bytes.next() {
        let byte = match read_result {
            Ok(byte) => byte,
            Err(e) => return Err(e),
        };

        if was_cr {
            if byte == LF_BYTE {
                break;
            } else {
                was_cr = false;
                buf.push(CR_BYTE);
            }
        }

        if byte == CR_BYTE {
            was_cr = true;
            continue;
        }

        buf.push(byte);
    }

    from_utf8(&buf)
        .map(|s| s.to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

impl Request {
    pub fn from_stream<R>(reader_stream: R) -> Result<Self, RequestReadError>
    where
        R: Read,
    {
        let mut bytes = reader_stream.bytes();

        // First line
        let first_line = read_line(&mut bytes)?;

        let mut first_line_parts = first_line.split_whitespace();

        let method_string = first_line_parts
            .next()
            .ok_or(RequestReadError::MalformedRequest(first_line.to_string()))?;
        let method = Method::from_str(method_string)
            .map_err(|_| RequestReadError::InvalidMethod(method_string.to_string()))?;
        let uri_string = first_line_parts
            .next()
            .ok_or(RequestReadError::MalformedRequest(first_line.to_string()))?
            .to_string();
        let uri = Uri::from_str(&uri_string).map_err(|_| RequestReadError::InvalidUri(uri_string))?;
        let version = first_line_parts
            .next()
            .ok_or(RequestReadError::MalformedRequest(first_line.to_string()))?
            .to_string();

        // Parse headers until we get an empty line

        let mut headers = HashMap::<String, String>::new();

        loop {
            let line = read_line(&mut bytes)?;

            if line.is_empty() {
                break;
            }

            let mut line_chars = line.chars();

            let mut header_name = String::new();

            loop {
                let c = line_chars
                    .next()
                    .ok_or(RequestReadError::MalformedHeader(line.to_string()))?;

                // Separates header name from value, so this means we are done with the header name
                if c == ':' {
                    break;
                }

                header_name.push(c);
            }

            // Get rid of extra space
            line_chars.next();

            headers.insert(clean_header(&header_name), line_chars.collect());
        }

        // Handle special headers

        let content_length = match headers.get("Content-Length") {
            Some(s) => usize::from_str(s).unwrap_or_default(),
            None => 0,
        };
        headers.remove("Content-Length");

        let content_type = match headers.get("Content-Type") {
            Some(s) => ContentType::from_str(s).unwrap_or_default(),
            None => ContentType::default(),
        };
        headers.remove("Content-Type");

        // Parse body if exists

        let mut body = Vec::new();

        if content_length > 0 {
            body = Vec::with_capacity(content_length);

            for i in 0..content_length {
                let byte = match bytes.next() {
                    Some(read_result) => match read_result {
                        Ok(byte) => byte,
                        Err(e) => return Err(RequestReadError::IoError(e)),
                    },
                    None => return Err(RequestReadError::InvalidContentLength((content_length, i + 1))),
                };

                body.push(byte);
            }
        }

        Ok(Request {
            body,
            content_type,
            headers,
            method,
            uri,
            version,
        })
    }

    pub fn content_length(&self) -> usize {
        self.body.len()
    }

    pub fn header(&self, name: &str) -> Option<String> {
        match clean_header(name).as_str() {
            "Content-Length" => Some(self.content_length().to_string()),
            "Content-Type" => Some(self.content_type.to_string()),
            _ => self.headers.get(name).cloned(),
        }
    }

    pub fn headers(&self) -> HashMap<String, String> {
        let mut headers = self.headers.clone();

        // Special headers
        headers.insert("Content-Length".to_string(), self.header("Content-Length").unwrap());
        headers.insert("Content-Type".to_string(), self.header("Content-Type").unwrap());

        headers
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        match clean_header(name).as_str() {
            "Content-Length" => (), // Need to update body
            "Content-Type" => self.content_type = ContentType::from_str(value).unwrap_or_default(),
            _ => {
                self.headers.insert(name.to_string(), value.to_string());
            }
        }
    }

    pub fn text(&self) -> Result<String, Utf8Error> {
        from_utf8(&self.body).map(|s| s.to_string())
    }
}
