#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

mod config;
mod constants;
mod errors;
mod handlers;
mod http;
mod router;
mod routes;
mod server;
mod utils;

use config::Config;
use errors::AppError;
use server::app_server;

pub type Result<T> = std::result::Result<T, AppError>;

fn check_directory(dir: &str) -> bool {
    let path = std::path::Path::new(dir);
    path.exists() && path.is_dir()
}

fn main() -> Result<()> {
    let config = Config::new();
    if !check_directory(&config.directory) {
        std::fs::create_dir(&config.directory)?;
    }
    app_server(Config::new())
}
