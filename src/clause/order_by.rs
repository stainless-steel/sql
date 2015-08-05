//! The `ORDER BY` clause.

use std::fmt::Debug;

use clause::Clause;
use definition::Column;
use expression::Expression;
use {Buffer, Result};

/// An `ORDER BY` clause.
#[derive(Debug, Default)]
pub struct OrderBy(Option<Vec<Box<Expression>>>);

/// An order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Order {
    /// The ascending order.
    Ascending,
    /// The descending order.
    Descending,
}

/// A type that can be ordered by.
pub trait Orderable: Debug where Self: Sized {
    /// The type produced after setting an order.
    type Output;

    /// Set the order.
    fn order(self, Option<Order>) -> Self::Output;

    /// Set the ascending order.
    fn ascending(self) -> Self::Output {
        self.order(Some(Order::Ascending))
    }

    /// Set the descending order.
    fn descending(self) -> Self::Output {
        self.order(Some(Order::Descending))
    }
}

impl OrderBy {
    #[doc(hidden)]
    pub fn append<T>(mut self, expression: T) -> Self where T: Expression + 'static {
        push!(self.0, Box::new(expression));
        self
    }
}

impl Clause for OrderBy {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        for expression in some!(self.0, expressions) {
            buffer.push(try!(expression.compile()));
        }
        Ok(format!("ORDER BY {}", buffer.join(", ")))
    }
}

impl Orderable for Column {
    type Output = (Column, Option<Order>);

    #[inline]
    fn order(self, order: Option<Order>) -> Self::Output {
        (self, order)
    }
}

impl<'l> Orderable for &'l str {
    type Output = (String, Option<Order>);

    #[inline]
    fn order(self, order: Option<Order>) -> Self::Output {
        (self.to_string(), order)
    }
}

impl Orderable for usize {
    type Output = (usize, Option<Order>);

    #[inline]
    fn order(self, order: Option<Order>) -> Self::Output {
        (self, order)
    }
}

impl<T: Expression> Expression for (T, Option<Order>) {
    fn compile(&self) -> Result<String> {
        let main = try!(self.0.compile());
        Ok(match self.1 {
            Some(Order::Ascending) => format!("{} ASC", main),
            Some(Order::Descending) => format!("{} DESC", main),
            _ => main,
        })
    }
}

#[cfg(test)]
mod tests {
    use clause::Clause;
    use prelude::*;

    macro_rules! new(
        ($first:expr) => (super::OrderBy::default().append($first));
    );

    #[test]
    fn ascending() {
        let clause = new!("foo".ascending());
        assert_eq!(clause.compile().unwrap(), "ORDER BY foo ASC");

        let clause = new!(column("foo").ascending());
        assert_eq!(clause.compile().unwrap(), "ORDER BY `foo` ASC");
    }

    #[test]
    fn descending() {
        let clause = new!("foo".descending());
        assert_eq!(clause.compile().unwrap(), "ORDER BY foo DESC");

        let clause = new!(column("foo").descending());
        assert_eq!(clause.compile().unwrap(), "ORDER BY `foo` DESC");
    }

    #[test]
    fn unspecified() {
        let clause = new!("foo");
        assert_eq!(clause.compile().unwrap(), "ORDER BY foo");

        let clause = new!(column("foo"));
        assert_eq!(clause.compile().unwrap(), "ORDER BY `foo`");
    }

    #[test]
    fn append() {
        let clause = new!("foo").append(column("bar").ascending())
                                .append("baz".to_string().descending());

        assert_eq!(clause.compile().unwrap(), "ORDER BY foo, `bar` ASC, baz DESC");
    }
}
