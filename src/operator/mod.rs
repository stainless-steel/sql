//! Operators.

use Result;

/// An operator.
pub trait Operator {
    /// Compile the operator.
    fn compile(&self) -> Result<String>;
}

pub mod like;

pub use self::like::Likable;
pub use self::like::Like;
pub use self::like::Pattern;

/// Helper functions.
pub mod helper {
    use expression::Expression;
    use super::Like;

    /// Create a `LIKE` operator.
    #[inline]
    pub fn like<T: 'static + Expression>(object: T) -> Like {
        Like::new(object)
    }
}
