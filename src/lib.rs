//! A constructor of SQL statements.
//!
//! ## Example
//!
//! ```
//! use sql::prelude::*;
//!
//! // CREATE TABLE `users` (`id` INTEGER NOT NULL, `name` TEXT, `photo` BLOB)
//! println!("{}", create_table("users").column(column("id").integer().not_null())
//!                                     .column(column("name").string())
//!                                     .column(column("photo").binary())
//!                                     .compile().unwrap());
//!
//! // INSERT INTO `users` (`id`, `name`) VALUES (?, ?), (?, ?)
//! println!("{}", insert_into("users").columns(&["id", "name"]).batch(2)
//!                                    .compile().unwrap());
//!
//! // SELECT * FROM `users` WHERE `name` LIKE 'A%'
//! println!("{}", select_from("users").so_that(column("name").like("A%"))
//!                                    .compile().unwrap());
//!
//! // SELECT * FROM `users` ORDER BY `name` DESC
//! println!("{}", select_from("users").order_by(column("name").descending())
//!                                    .compile().unwrap());
//!
//! // SELECT `name`, `photo` FROM `users` LIMIT 1
//! println!("{}", select_from("users").columns(&["name", "photo"]).limit(1)
//!                                    .compile().unwrap());
//! ```

use std::{error, fmt, result};

/// An error.
pub struct Error(String);

/// A result.
pub type Result<T> = result::Result<T, Error>;

/// A data type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// The binary type.
    Binary,
    /// The floating-point type.
    Float,
    /// The integer type.
    Integer,
    /// The string type.
    String,
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.0
    }
}

macro_rules! raise(
    ($message:expr) => (
        return Err(::Error($message.to_string()));
    );
);

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => raise!(error),
        }
    );
);

pub mod grammar;
pub mod language;
pub mod prelude;
