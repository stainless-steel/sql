//! Clauses.

use std::fmt::Debug;

use Result;

/// A clause.
pub trait Clause: Debug {
    /// Compile the clause.
    fn compile(&self) -> Result<String>;
}

#[doc(hidden)]
pub mod order_by;

pub use self::order_by::{OrderBy, Order, Orderable};

/// Helper functions.
pub mod helper {
    use expression::Expression;
    use super::OrderBy;

    /// Create an `ORDER BY` clause.
    #[inline]
    pub fn order_by<T: 'static + Expression>(order: T) -> OrderBy {
        OrderBy::new(order)
    }
}
