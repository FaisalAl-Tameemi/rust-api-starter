# Rust API Boilerplate with Actix & Diesel

Simple web API boilerplate built with Rust, using Actix as an HTTP server framework and Diesel for PostgreSQL database interactions.

## Features

- 🚀 Built with Actix-web - A powerful, pragmatic, and extremely fast web framework for Rust
- 🗄️ Diesel ORM integration with PostgreSQL
- 🔐 JWT-based authentication
- ✨ Request validation using actix-web-validator
- 🛠️ Error handling middleware
- 📝 Basic user management (create, list, get, login)
- 🔍 Basic post endpoints
- ⚙️ Environment-based configuration


## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Diesel CLI
- Docker / Docker Compose (optional)

### Setup

1. Start by cloning the repository and installing the dependencies:

```bash
git clone git@github.com:FaisalAl-Tameemi/rust-api-starter.git
cd rust-api-starter
cargo install
```

2. Create a `.env` file in the root of the project and add the following variables:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/rust_api_starter
```

> Note: The environment package uses both `.env` and `config.toml` read environment variables using the [`dotenvy`](https://crates.io/crates/dotenvy) crate. In the current setup, the `.env` file only needs to have the `DATABASE_URL` variable for the Diesel CLI to work, otherwise, the `config.toml` file is used to read the environment variables.

3. Start a PostgreSQL instance using your preferred method or use the `docker-compose.yml` file to start a PostgreSQL instance.

```bash
docker compose up -d db
```

> Note: edit the `.env` and `config.toml` files to match your database credentials and database name.

4. Run the database migrations

```bash
diesel migration run
```

5. Run the server

```bash
cargo run
```

or if you would like to run the server in watch mode, you can install [`cargo-watch`](https://crates.io/crates/cargo-watch) and run `cargo watch -x run`.


### File Structure

Within the `./src` directory, you will find the following files and directories:

```
./src
├── api
│   ├── mod.rs
│   ├── posts.rs
│   └── users.rs
├── config.rs
├── error.rs
├── lib.rs
├── main.rs
├── models
│   ├── mod.rs
│   └── user.rs
├── schema.rs
├── server.rs
└── util
    ├── auth.rs
    ├── db.rs
    ├── error.rs
    ├── mod.rs
    └── token.rs
```

Here's a brief overview of each file:

- `api/`: Contains the API routes and handlers.
- `config.rs`: Contains the configs parsed from `.env` and `config.toml`, it adds some basic type safety.
- `error.rs`: Contains the `AppError` struct which is used throughout the codebase.
- `models/`: Contains the database models and the DTO schemas used for request validation.
- `server.rs`: Contains the server routes and configuration.
- `util/`: Contains utility functions and modules.
    - `auth.rs`: Contains an Actix extractor to protect routes with JWT authentication.
    - `db.rs`: Contains the database connection helper. It uses the [`r2d2`](https://crates.io/crates/r2d2) crate to manage the database connection pool.
    - `error.rs`: Contains an error handler middleware for the server.
    - `token.rs`: Contains the JWT token generation and validation logic. It uses the [`jsonwebtoken`](https://crates.io/crates/jsonwebtoken) crate.


## License

This project is licensed under the MIT License.

MIT License

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

