//! Operators.

use definition::Column;
use expression;

/// A `LIKE` operator.
pub trait Like {
    /// Apply the operator.
    fn like<T: ToString>(self, T) -> expression::Like<Self>;
}

impl Like for Column {
    fn like<A: ToString>(self, value: A) -> expression::Like<Self> {
        expression::Like(self, value.to_string())
    }
}
