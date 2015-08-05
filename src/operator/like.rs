//! The `LIKE` operator.

use Result;
use definition::Column;
use expression::Expression;
use operator::Operator;

/// A `LIKE` operator.
#[derive(Debug, Default)]
pub struct Like(Option<Box<Expression>>);

/// A pattern to match against.
#[derive(Clone, Debug)]
pub struct Pattern(String);

/// A type that can be matched.
pub trait Likable {
    /// The type produced after setting a matcher.
    type Output;

    /// Set a matcher.
    fn like<T: ToString>(self, T) -> Self::Output;
}

impl Operator for Like {
    #[inline]
    fn compile(&self) -> Result<String> {
        some!(self.0, expression).compile()
    }
}

impl Likable for Column {
    type Output = (Column, Pattern);

    #[inline]
    fn like<T: ToString>(self, pattern: T) -> Self::Output {
        (self, Pattern(pattern.to_string()))
    }
}

impl<'l> Likable for &'l str {
    type Output = (String, Pattern);

    #[inline]
    fn like<T: ToString>(self, pattern: T) -> Self::Output {
        (self.to_string(), Pattern(pattern.to_string()))
    }
}

impl<T: Expression> Expression for (T, Pattern) {
    fn compile(&self) -> Result<String> {
        Ok(format!("{} LIKE '{}'", try!(self.0.compile()), (self.1).0))
    }
}
