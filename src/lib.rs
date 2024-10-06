use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

fn get_path_parts<'a>(s: &'a str) -> Vec<&'a str> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}

enum Route {
    Empty,
    Echo,
    UserAgent,
    Files,
    Unknown
}

impl From<&str> for Route {
    fn from(s: &str) -> Self {
        match s {
            "echo" => Self::Echo,
            "user-agent" => Self::UserAgent,
            "files" => Self::Files,
            "/" => Self::Empty,
            _ => Self::Unknown
        }
    }
}

enum Method {
    Get,
    Post,
}

impl From<Option<&str>> for Method {
    fn from(o: Option<&str>) -> Self {
        match o {
            Some("GET") => Self::Get,
            Some("POST") => Self::Post,
            // TODO: do I want to panic here?
            _ => panic!("Requires an HTTP method"),
        }
    }
}

struct Request {
    // TODO: avoiding lifetimes, stop doing this!!
    method: Method,
    route: Route,
    path: String,
    headers: HashMap<String, String>,
}

impl TryFrom<BufReader<&TcpStream>> for Request {
    type Error = String;
    fn try_from(value: BufReader<&TcpStream>) -> Result<Self, Self::Error> {
        let err = "Couldn't get next line";
        let mut lines = value.lines();
        let start_line = match lines.next() {
            Some(Ok(s)) => s,
            _ => {
                return Err(err.to_owned());
            }
        };
        let mut start_parts = start_line.split_whitespace();
        let method = Method::from(start_parts.next());
        let path = match start_parts.next() {
            Some(s) => s.to_owned(),
            _ => {
                return Err(err.to_owned());
            }
        };
        let path_parts = get_path_parts(path.as_str());

        let route = if path_parts.is_empty() {
            Route::from("/")
        } else {
            Route::from(path_parts[0])
        };

        let mut headers = HashMap::new();

        while let Some(Ok(header_line)) = lines.next() {
            let key_value = header_line.split_terminator(":").collect::<Vec<&str>>();
            if key_value.is_empty() {
                // I think we have reached the body at this point
                break;
            }
            let key = key_value[0];
            let value = key_value[1].trim();
            let _ = headers.insert(key.to_owned(), value.to_owned());
        }

        Ok(Self {
            route,
            path,
            method,
            headers,
        })
    }
}

// TODO: sort out errors
fn four_oh_four(s: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    s.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())?;
    Ok(())
}

pub fn handle_request(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(&stream);
    let req = Request::try_from(reader)?;

    match (req.method, req.route) {
        (Method::Get, Route::Empty) => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
            Ok(())
        }
        // TODO: extract this...
        (Method::Get, Route::Echo) => {
            let body = get_path_parts(req.path.as_str())[1];
            let content_length = body.len();
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    content_length, body
                )
                .as_bytes(),
            )?;
            Ok(())
        }
        (Method::Get, Route::UserAgent) => {
            let body = format!(
                "{}",
                req.headers.get("User-Agent").unwrap_or(&String::from(""))
            );
            let content_length = body.len();
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    content_length, body
                )
                .as_bytes(),
            )?;
            Ok(())
        }
        _ => four_oh_four(&mut stream),
    }
}
