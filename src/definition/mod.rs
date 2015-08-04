//! Definitions.

use Result;

/// A definition.
pub trait Definition {
    /// Compile the definition.
    fn compile(&self) -> Result<String>;
}

mod column;

pub use self::column::Column;

/// Helper functions.
pub mod helper {
    use super::Column;

    /// Create a column definition.
    #[inline]
    pub fn column<T: ToString>(name: T) -> Column {
        Column::new(name)
    }
}
