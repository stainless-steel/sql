//! Expressions.

use std::fmt::Debug;

use Result;

/// An expression.
pub trait Expression: Debug {
    /// Compile the expression.
    fn compile(&self) -> Result<String>;
}

/// A `LIKE` expression.
#[derive(Clone, Debug)]
pub struct Like<T: Expression>(pub T, pub String);

impl<T: Expression> Expression for Like<T> {
    fn compile(&self) -> Result<String> {
        Ok(format!("{} LIKE '{}'", try!(self.0.compile()), self.1))
    }
}
