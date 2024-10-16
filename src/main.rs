#![warn(clippy::style, clippy::complexity, clippy::perf)]

use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub mod errors;
pub mod handlers;
pub mod http;
pub mod router;
pub mod routes;
pub mod utils;

use crate::router::request_router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    // TODO: I will most likely just use clap here when I'm cleaning up...
    // program name
    let _ = args.next();
    // --directory flag
    let _ = args.next();
    let partial_file_path = Arc::new(Mutex::new(args.next()));
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        let path = Arc::clone(&partial_file_path);
        match stream {
            Ok(stream) => {
                // TODO: naive!!
                std::thread::spawn(move || {
                    request_router(stream, path);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
