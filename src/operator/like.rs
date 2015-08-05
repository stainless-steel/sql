//! The `LIKE` operator.

use Result;
use definition::Column;
use expression::Expression;
use operator::Operator;

/// A `LIKE` operator.
#[derive(Debug, Default)]
pub struct Like {
    object: Option<Box<Expression>>,
}

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

impl Like {
    /// Create a `LIKE` operator.
    #[inline]
    pub fn new<T: 'static + Expression>(object: T) -> Self {
        Like::default().set(object)
    }

    /// Set the expression.
    pub fn set<T: 'static + Expression>(mut self, object: T) -> Self {
        self.object = Some(Box::new(object));
        self
    }
}

impl Operator for Like {
    #[inline]
    fn compile(&self) -> Result<String> {
        some!(self.object).compile()
    }
}

impl Likable for Column {
    type Output = (Column, Pattern);

    #[inline]
    fn like<T: ToString>(self, pattern: T) -> Self::Output {
        (self, Pattern(pattern.to_string()))
    }
}

impl Likable for String {
    type Output = (String, Pattern);

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
