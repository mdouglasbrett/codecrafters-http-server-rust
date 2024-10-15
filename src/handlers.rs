use std::io::prelude::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::http::{Request, Response};
use crate::utils::{get_encoding, get_path_parts, read_file, write_file};

// TODO: make more meaningful errors!!
type HandlerError = Box<dyn std::error::Error>;

pub fn handle_empty(s: &mut TcpStream) -> Result<(), HandlerError> {
    s.write_all(&Response::Ok(None).to_vec())?;
    Ok(())
}

pub fn handle_echo(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let encoding = get_encoding(&r.headers);
    let body = get_path_parts(r.path.as_str())[1];
    s.write_all(
        &Response::Ok(Some((body.to_owned(), "text/plain".to_owned(), encoding))).to_vec(),
    )?;
    Ok(())
}

pub fn handle_user_agent(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let body = r
        .headers
        .get("User-Agent")
        .unwrap_or(&String::from(""))
        .to_owned();
    let encoding = get_encoding(&r.headers);
    s.write_all(&Response::Ok(Some((body, "text/plain".to_owned(), encoding))).to_vec())?;
    Ok(())
}

pub fn handle_get_file(
    s: &mut TcpStream,
    r: &Request,
    fp: Arc<Mutex<Option<String>>>,
) -> Result<(), HandlerError> {
    let filename = get_path_parts(&r.path)[1];
    let contents = read_file(fp, filename);
    let encoding = get_encoding(&r.headers);
    if contents.is_none() {
        handle_unknown(s)
    } else {
        if contents.is_some() {
            s.write_all(
                &Response::Ok(Some((
                    contents.unwrap(),
                    "application/octet-stream".to_owned(),
                    encoding,
                )))
                .to_vec(),
            )?;
        }
        Ok(())
    }
}

pub fn handle_post_file(
    s: &mut TcpStream,
    r: &Request,
    fp: Arc<Mutex<Option<String>>>,
) -> Result<(), HandlerError> {
    let filename = get_path_parts(&r.path)[1];
    write_file(fp, filename, r)?;
    s.write_all(&Response::Created.to_vec())?;
    Ok(())
}

pub fn handle_unknown(s: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    s.write_all(&Response::NotFound.to_vec())?;
    Ok(())
}
