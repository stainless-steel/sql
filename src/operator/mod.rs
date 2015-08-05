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
