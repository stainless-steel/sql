//! Operators.

use definition::Column;
use expression::Like;

/// A type that can be matched.
pub trait Likable {
    /// The type produced after setting a matcher.
    type Output;

    /// Set a matcher.
    fn like<T: ToString>(self, T) -> Self::Output;
}

impl Likable for Column {
    type Output = Like<Self>;

    fn like<A: ToString>(self, value: A) -> Self::Output {
        Like(self, value.to_string())
    }
}
