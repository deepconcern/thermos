use std::{collections::HashMap, fmt::Display, fs::File, io::{self, Read, Write}, path::Path, str::{from_utf8, FromStr}};

use crate::http::content_type::{ContentType, TextType};

use super::status::Status;

const DEFAULT_HTTP_VERSION: &str = "HTTP/1.1";
const SERVER_NAME: &str = "Thermos";

fn default_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();

    headers.insert("Server".to_string(), SERVER_NAME.to_string());

    headers
}

#[derive(Clone, Debug)]
pub struct Response {
    pub body: Vec<u8>,
    pub content_type: ContentType,
    pub headers: HashMap<String, String>,
    pub status: Status,
    pub version: String,
}

impl Response {
    pub fn bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = Vec::new();

        // First line
        write!(&mut buf, "{} {}\r\n", self.version, self.status)?;

        // Headers
        for (header_name, header_value) in self.headers.iter() {
            let mut formatted_name = String::with_capacity(header_name.len());

            for (i, c) in header_name.chars().enumerate() {
                if i == 0 {
                    formatted_name.push(c.to_ascii_uppercase());
                } else {
                    formatted_name.push(c.to_ascii_lowercase())
                }
            }

            write!(&mut buf, "{}: {}\r\n", formatted_name, header_value)?;
        }

        // Body
        write!(&mut buf, "\r\n")?;
        for body_byte in self.body.iter() {
            buf.push(*body_byte);
        }

        Ok(buf)
    }

    pub fn with_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(&path)?;

        let extension = path.as_ref().extension().unwrap_or_default();

        let mut bytes = Vec::new();

        for read_result in file.bytes() {
            let byte = read_result?;

            bytes.push(byte);
        }

        let content_type = ContentType::Text(match TextType::from_str(extension.to_str().unwrap_or_default()) {
            Ok(text_type) => text_type,
            Err(_) => TextType::default(),
        });

        Ok(Self {
            body: bytes,
            content_type,
            ..Default::default()
        })
    }

    pub fn with_text(text: &str) -> Self {
        Self {
            body: text.bytes().collect(),
            content_type: ContentType::Text(TextType::Plain),
            ..Default::default()
        }
    }

    pub fn with_status(status: Status) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            body: Vec::new(),
            content_type: ContentType::default(),
            headers: default_headers(),
            status: Status::Ok,
            version: DEFAULT_HTTP_VERSION.to_string(),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.bytes().map_err(|_| std::fmt::Error)?;
        let s = from_utf8(&bytes).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", s)
    }
}