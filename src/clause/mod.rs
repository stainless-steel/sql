//! Clauses.

use std::fmt::Debug;

use Result;

/// A clause.
pub trait Clause: Debug {
    /// Compile the clause.
    fn compile(&self) -> Result<String>;
}

pub mod order_by;

pub use self::order_by::Order;
pub use self::order_by::OrderBy;
pub use self::order_by::Orderable;

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
