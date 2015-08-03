//! Operations.

use expression;

/// A `LIKE` operation.
pub trait Like {
    /// Apply the operation.
    fn like<T: ToString>(self, T) -> expression::Like<Self>;
}
