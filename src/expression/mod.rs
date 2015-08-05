//! Expressions.

use std::fmt::Debug;

use Result;

/// An expression.
pub trait Expression: Debug {
    /// Compile the expression.
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
