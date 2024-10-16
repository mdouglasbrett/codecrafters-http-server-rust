use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use flate2::{write::GzEncoder, Compression};

use crate::errors::{ClientError, ServerError, RequestError};
use crate::routes::Route;
use crate::utils::get_path_parts;

pub enum Method {
    Get,
    Post,
    Unsupported(ServerError),
}

impl From<Option<&str>> for Method {
    fn from(o: Option<&str>) -> Self {
        match o {
            Some("GET") => Self::Get,
            Some("POST") => Self::Post,
            // TODO: do I want to panic here?
            _ => Self::Unsupported(ServerError::NotImplemented),
        }
    }
}

// TODO: this is a temporary solution. Is there something better?
#[derive(Debug)]
pub enum HeaderField {
    Single(String),
    Multiple(Vec<String>),
}

pub struct Request {
    pub method: Method,
    pub route: Route,
    // https://steveklabnik.com/writing/when-should-i-use-string-vs-str/
    pub path: String,
    pub headers: HashMap<String, HeaderField>,
    pub body: Vec<u8>,
}

// TODO: error handling
impl TryFrom<&TcpStream> for Request {
    type Error = RequestError;
    fn try_from(value: &TcpStream) -> Result<Self, Self::Error> {
        let mut buf = BufReader::new(value);
        let mut start_line = String::new();
        let _ = buf.read_line(&mut start_line)?;
        let mut start_parts = start_line.split_whitespace();
        let method = match Method::from(start_parts.next()) {
            Method::Unsupported(err) => {
                return Err(err.into());
            }
            Method::Get => Method::Get,
            Method::Post => Method::Post,
        };
        let path = match start_parts.next() {
            Some(s) => s.to_owned(),
            None => {
                return Err(ClientError::BadRequest.into());
            }
        };
        let path_parts = get_path_parts(path.as_str());

        let route = if path_parts.is_empty() {
            Route::from("/")
        } else {
            Route::from(path_parts[0])
        };

        let mut headers = HashMap::new();

        loop {
            let mut header_line = String::new();
            let _ = buf.read_line(&mut header_line)?;
            let trimmed_header_line = header_line.trim();
            if trimmed_header_line.is_empty() {
                // I think we have reached the body at this point
                break;
            }
            let key_value = trimmed_header_line
                .split_terminator(":")
                .collect::<Vec<&str>>();
            let key = key_value[0];
            let raw_value = key_value[1].trim();
            let value = if key == "Accept-Encoding" {
                HeaderField::Multiple(raw_value.split(", ").map(|s| s.to_owned()).collect())
            } else {
                HeaderField::Single(raw_value.to_owned())
            };
            let _ = headers.insert(key.to_owned(), value);
        }

        let mut body_buf: Vec<u8> = vec![];

        // If there's no content length, do not attempt to parse the body
        if let Some(len) = headers.get("Content-Length") {
            match len {
                HeaderField::Single(len) => {
                    let len = len.parse::<u64>()?;
                    buf.take(len).read_to_end(&mut body_buf)?;
                }
                HeaderField::Multiple(_) => {
                    return Err(ClientError::BadRequest.into());
                }
            }
        }

        Ok(Self {
            route,
            path,
            method,
            headers,
            body: body_buf,
        })
    }
}

// TODO: probably going to have to expand on some of these (client err,
// server err)
pub enum Response {
    Ok(Option<(String, String, Option<String>)>),
    NotFound,
    Created,
    ClientError(ClientError),
    ServerError(ServerError),
}

// TODO: this could error out
impl Response {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Ok(Some((body, mime, encoding))) => {
                let content = if encoding.is_some() {
                    let mut b = GzEncoder::new(Vec::new(), Compression::default());
                    // TODO: handle errors
                    let _ = b.write_all(body.as_bytes());
                    let compressed_body = b.finish();
                    if let Ok(bytes) = compressed_body {
                        bytes
                    } else {
                        todo!()
                    }
                } else {
                    body.as_bytes().to_vec()
                };
                let mut response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {content_length}\r\n{content_encoding}\r\n",
                content_type = mime,
                content_encoding = match encoding {
                    Some(e) => format!("Content-Encoding: {}\r\n", e),
                    None => "".to_owned()
                },
                content_length = content.len(),)
                    .as_bytes()
                    .to_vec();
                if !content.is_empty() {
                    response.extend_from_slice(&content);
                }

                response
            }
            Self::Ok(None) => "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec(),
            Self::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_vec(),
            Self::Created => "HTTP/1.1 201 Created\r\n\r\n".as_bytes().to_vec(),
            Self::ServerError(err) => format!("HTTP/1.1 {}\r\n\r\n", err.to_owned())
                .as_bytes()
                .to_vec(),
            Self::ClientError(err) => format!("HTTP/1.1 {}\r\n\r\n", err.to_owned())
                .as_bytes()
                .to_vec(),
        }
    }
}
