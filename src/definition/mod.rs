//! Definitions.

use Result;

/// A definition.
pub trait Definition {
    /// Compile the definition.
    fn compile(&self) -> Result<String>;
}

mod column;

pub use self::column::Column;
