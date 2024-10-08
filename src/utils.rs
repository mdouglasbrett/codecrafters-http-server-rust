use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::request::Request;
use crate::response::Status;

pub fn get_path_parts(s: &str) -> Vec<&str> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}

pub fn get_file_contents(fp: Arc<Mutex<Option<String>>>, filename: &str) -> Option<String> {
    // TODO: I really don't like this unwrap/clone/unwrap dance
    let path = Path::new(&fp.lock().unwrap().clone().unwrap()).join(filename);
    if let Ok(f) = fs::read_to_string(path) {
        Some(f)
    } else {
        None
    }
}

pub fn write_file(fp: Arc<Mutex<Option<String>>>, filename: &str, req: &Request) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn get_response(status: Status, body: Option<(String, String)>) -> Vec<u8> {
    match status {
        Status::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_vec(),
        Status::Ok => {
            if let Some((body, mime)) = body {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                    mime,
                    body.len(),
                    body
                )
                .as_bytes()
                .to_vec()
            } else {
                "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec()
            }
        },
        Status::Created => "HTTP/1.1 201 Created\r\n\r\n".as_bytes().to_vec(),
    }
}

