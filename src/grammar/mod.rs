//! Grammar elements.

use std::fmt::Debug;

use Result;

/// A clause.
pub trait Clause: Debug {
    /// Compile the clause.
    fn compile(&self) -> Result<String>;
}

/// A condition.
pub trait Condition: Debug {
    /// Compile the condition.
    fn compile(&self) -> Result<String>;
}

/// A definition.
pub trait Definition: Debug {
    /// Compile the definition.
    fn compile(&self) -> Result<String>;
}

/// An expression.
pub trait Expression: Debug {
    /// Compile the expression.
    fn compile(&self) -> Result<String>;
}

/// An operation.
pub trait Operation: Debug {
    /// Compile the operation.
    fn compile(&self) -> Result<String>;
}

/// A statement.
pub trait Statement: Debug {
    /// Compile the statement.
    fn compile(&self) -> Result<String>;
}

impl<'l> Expression for &'l str {
    #[inline]
    fn compile(&self) -> Result<String> {
        Ok(self.to_string())
    }
}

impl Expression for String {
    #[inline]
    fn compile(&self) -> Result<String> {
        Ok(self.clone())
    }
}

impl Expression for usize {
    #[inline]
    fn compile(&self) -> Result<String> {
        Ok(self.to_string())
    }
}

impl<T: Operation> Condition for T {
    #[inline]
    fn compile(&self) -> Result<String> {
        Operation::compile(self)
    }
}

pub mod clause;
pub mod definition;
pub mod operation;
pub mod statement;
