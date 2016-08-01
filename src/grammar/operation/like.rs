//! The `LIKE` operation.

use Result;
use grammar::definition::Column;
use grammar::{Condition, Expression, Operation};

/// A `LIKE` operation.
#[derive(Debug)]
pub struct Like(Box<Expression>, String);

/// An object that can be matched.
pub trait Likable {
    /// Set a matcher.
    fn like<T: ToString>(self, T) -> Like;
}

impl Condition for Like {
    #[inline]
    fn compile(&self) -> Result<String> {
        Operation::compile(self)
    }
}

impl Operation for Like {
    #[inline]
    fn compile(&self) -> Result<String> {
        Ok(format!("{} LIKE '{}'", try!(self.0.compile()), self.1))
    }
}

impl Likable for Column {
    #[inline]
    fn like<T: ToString>(self, pattern: T) -> Like {
        Like(Box::new(self), pattern.to_string())
    }
}

impl<'l> Likable for &'l str {
    #[inline]
    fn like<T: ToString>(self, pattern: T) -> Like {
        Like(Box::new(self.to_string()), pattern.to_string())
    }
}

#[cfg(test)]
mod tests {
    use grammar::Operation;
    use prelude::*;

    #[test]
    fn from_column() {
        assert_eq!(column("foo").like("bar").compile().unwrap(), "`foo` LIKE 'bar'");
    }

    #[test]
    fn from_string() {
        assert_eq!("foo".like("bar").compile().unwrap(), "foo LIKE 'bar'");
    }
}
