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
//! // SELECT `name`, `photo` FROM `users` LIMIT 1
//! println!("{}", select_from("users").columns(&["name", "photo"]).limit(1)
//!                                    .compile().unwrap());
//! ```

use std::{error, fmt, result};

struct Buffer(Vec<String>);

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

impl Buffer {
    fn new() -> Buffer {
        Buffer(vec![])
    }

    fn push<T: ToString>(&mut self, chunk: T) -> &mut Self {
        self.0.push(chunk.to_string());
        self
    }

    fn join(self, delimiter: &str) -> String {
        let mut result = String::new();
        for (i, chunk) in self.0.iter().enumerate() {
            if i > 0 {
                result.push_str(delimiter)
            }
            result.push_str(chunk);
        }
        result
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
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

macro_rules! some(
    ($this:ident.$field:ident) => (
        match $this.$field {
            Some(ref value) => value,
            _ => raise!(concat!("expected “", stringify!($field), "” to be set")),
        }
    );
);

macro_rules! push(
    ($this:ident.$field:ident, $value:expr) => (
        match $this.$field {
            Some(ref mut collection) => {
                collection.push($value);
            },
            _ => {
                $this.$field = Some(vec![$value]);
            },
        }
    );
);

pub mod clause;
pub mod definition;
pub mod expression;
pub mod operation;
pub mod statement;

pub mod prelude;
