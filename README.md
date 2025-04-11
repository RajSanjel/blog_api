# Rust Blog Platform API

A simple blogging API built with **Rust**, **Axum**, and **SQLx**. This project aims to implement a RESTful API for a blog platform, featuring functionality for handling posts and comments. The API connects to a PostgreSQL database and supports UUIDs and timestamps.

## Features

-   **Post Management**: Create, read, update, and delete blog posts.
-   **UUIDs**: Use UUID for primary keys.
-   **Timestamps**: Automatically set created and updated timestamps.
-   **Database**: Connects to a PostgreSQL database using `sqlx`.
-   **Axum**: Framework for building the API.

## Tech Stack

-   [Rust](https://www.rust-lang.org/) - Systems programming language.
-   [Axum](https://axum.rs/) - Web framework built on top of Tokio.
-   [SQLx](https://docs.rs/sqlx) - Asynchronous, compile-time checked SQL library.
-   [PostgreSQL](https://www.postgresql.org/) - Relational database for storing blog data.
-   [UUID](https://docs.rs/uuid) - For generating unique identifiers.
-   [chrono](https://crates.io/crates/chrono) - For working with dates and times.

## Getting Started

This project is still in development. To run it:

```bash
cargo run
```
