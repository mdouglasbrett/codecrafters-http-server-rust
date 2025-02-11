use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use crate::{
    common::headers as header_fields,
    errors::{ClientError, ServerError},
    router::Route,
    utils::get_path_parts,
    Result,
};

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Unknown(ClientError),
    Unsupported(ServerError),
}

impl From<Option<&str>> for Method {
    fn from(o: Option<&str>) -> Self {
        match o {
            Some("GET") => Self::Get,
            Some("POST") => Self::Post,
            // Maybe tomorrow...
            Some("PUT") | Some("PATCH") | Some("OPTIONS") | Some("HEAD") | Some("DELETE")
            | Some("CONNECT") | Some("TRACE") => Self::Unsupported(ServerError::NotImplemented),
            _ => Self::Unknown(ClientError::BadRequest),
        }
    }
}

// TODO: this is a temporary solution. Is there something better?
#[derive(Debug, PartialEq)]
pub enum HeaderField {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub route: Route,
    // https://steveklabnik.com/writing/when-should-i-use-string-vs-str/
    pub path: String,
    pub headers: HashMap<String, HeaderField>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn try_new<T: Read>(value: &mut T) -> Result<Self> {
        let mut buf = BufReader::new(value);
        let mut start_line = String::new();
        let _ = buf.read_line(&mut start_line)?;
        let mut start_parts = start_line.split_whitespace();
        let method = match Method::from(start_parts.next()) {
            Method::Unsupported(err) => {
                return Err(err.into());
            }
            Method::Unknown(err) => {
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
            let value = if key == header_fields::ACCEPT_ENCODING {
                HeaderField::Multiple(raw_value.split(", ").map(|s| s.to_owned()).collect())
            } else {
                HeaderField::Single(raw_value.to_owned())
            };
            let _ = headers.insert(key.to_owned(), value);
        }

        let mut body_buf: Vec<u8> = vec![];

        if route == Route::Echo && path_parts.len() > 1 {
            body_buf.extend(path_parts[1].as_bytes());
        } else {
            // If there's no content length, do not attempt to parse the body
            if let Some(len) = headers.get(header_fields::CONTENT_LENGTH) {
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

#[cfg(test)]
mod tests {

    mod request {
        use crate::errors::{AppError, ClientError};
        use crate::http::request::{Method::Get, Request};
        use crate::router::Route::Echo;
        use std::collections::HashMap;

        #[test]
        fn handles_http_request() {
            let req = b"GET /echo/abc HTTP/1.1\r\n\r\n";
            let expected = Request {
                method: Get,
                route: Echo,
                path: "/echo/abc".to_owned(),
                body: b"abc".to_vec(),
                headers: HashMap::new(),
            };
            assert_eq!(expected, Request::try_new(&mut req.as_slice()).unwrap());
        }

        #[test]
        fn handles_bad_request() {
            let req = b"/echo/abc\r\n\r\n";
            assert_eq!(
                AppError::Client(ClientError::BadRequest),
                Request::try_new(&mut req.as_slice()).unwrap_err()
            );
        }
    }
}
