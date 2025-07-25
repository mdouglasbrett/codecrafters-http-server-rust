# HTTP Server in Rust

This project is an HTTP server written in Rust, created as part of the Codecrafters "Build your own HTTP server" challenge. The server supports basic HTTP functionalities such as handling GET and POST requests, serving static files, and echoing request bodies.

> [!CAUTION]
> This project is for my personal learning purposes. You really shouldn't even _think_ that this is worth using anywhere.

## Features

- **GET /echo/:message**: Echoes the message provided in the URL.
- **GET /user-agent**: Returns the `User-Agent` header from the request.
- **GET /files/:filename**: Serves static files from a specified directory.
- **POST /files/:filename**: Saves the request body as a file in the specified directory.
- **Gzip Compression**: Supports gzip compression for responses if requested by the client.
- **Thread Pool**: Handles concurrent connections using a fixed-size thread pool for improved performance under load.

## Project Structure

- `src/config.rs`: Configuration handling for the server.
- `src/errors.rs`: Custom error types for the server.
- `src/handlers.rs`: Request handlers for different routes.
- `src/http/mod.rs`: HTTP types and re-exports.
- `src/http/request.rs`: HTTP request parsing.
- `src/http/response.rs`: HTTP response generation.
- `src/main.rs`: Entry point of the application.
- `src/router.rs`: Request routing logic.
- `src/server/app_server.rs`: Server setup and connection handling.
- `src/server/thread_pool.rs`: Thread pool implementation for handling concurrent connections.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### Installation

1. Fork and clone the repository:

    ```sh
    git clone https://github.com/yourusername/http-server-rust.git
    cd http-server-rust
    ```

2. Build the project:

    ```sh
    cargo build
    ```

### Running the Server

To run the server, use the following command:

```sh
cargo run -- [-t | --target_dir=TARGET_DIR] [-a | --address=ADDRESS]
```

- `TARGET_DIR`: Directory to serve and save files (default/root: `/tmp`).
- `ADDRESS`: Address to bind the server to (default: `127.0.0.1:4221`).

Example:

```sh
cargo run -- --target_dir=/path/to/dir --address=127.0.0.1:8080
```
<!--
### Testing

> [!WARNING]
> Tests are currently being rewritten, this project does not use TDD

The project includes unit tests for various components. To run the tests, use the following command:

```sh
cargo test
```

This will execute all the tests and display the results.
-->

## Usage

### Endpoints

- **GET /echo/:message**

    Echoes the message provided in the URL.

    ```sh
    curl http://127.0.0.1:4221/echo/hello
    ```

- **GET /user-agent**

    Returns the `User-Agent` header from the request.

    ```sh
    curl -H "User-Agent: MyTestAgent" http://127.0.0.1:4221/user-agent
    ```

- **GET /files/:filename**

    Serves static files from the specified directory.

    ```sh
    curl http://127.0.0.1:4221/files/test.txt
    ```

- **POST /files/:filename**

    Saves the request body as a file in the specified directory.

    ```sh
    curl -X POST -d "File content" http://127.0.0.1:4221/files/test.txt
    ```

<!--
TODO:

- [ ] add more test coverage/fix existing tests
- [ ] add documentation
- [ ] add benchmarks? what would the benchmarks prove? this is a toy
- [ ] fork and refactor to use async/await - is that a TODO for _this_ project?
- [ ] clean up all the TODOs!
-->
