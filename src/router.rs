use std::{
    io::BufReader,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::handlers::{handle_echo, handle_empty, handle_get_file, handle_unknown, handle_user_agent, handle_post_file};
use crate::request::{Method, Request, Route};

pub fn request_router(
    mut stream: TcpStream,
    file_path: Arc<Mutex<Option<String>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(&stream);
    let req = Request::try_from(reader)?;

    match (&req.method, &req.route) {
        (Method::Get, Route::Empty) => handle_empty(&mut stream),
        (Method::Get, Route::Echo) => handle_echo(&mut stream, &req),
        (Method::Get, Route::UserAgent) => handle_user_agent(&mut stream, &req),
        (Method::Get, Route::Files) => handle_get_file(&mut stream, &req, file_path),
        (Method::Post, Route::Files) => handle_post_file(&mut stream, &req, file_path),
        _ => handle_unknown(&mut stream),
    }
}
