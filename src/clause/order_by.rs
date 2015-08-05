use std::fmt::Debug;

use clause::Clause;
use definition::Column;
use expression::Expression;
use {Buffer, Result};

/// An `ORDER BY` clause.
#[derive(Debug, Default)]
pub struct OrderBy {
    parts: Option<Vec<Box<Expression>>>,
}

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
    /// Create an `ORDER BY` clause.
    #[inline]
    pub fn new<T: 'static + Expression>(order: T) -> OrderBy {
        OrderBy::default().and(order)
    }

    /// Add an order.
    pub fn and<T: 'static + Expression>(mut self, value: T) -> Self {
        push!(self.parts, Box::new(value));
        self
    }
}

impl Clause for OrderBy {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        for part in some!(self.parts) {
            buffer.push(try!(part.compile()));
        }
        Ok(format!("ORDER BY {}", buffer.join(", ")))
    }
}

impl<T: Expression> Orderable for (T, Option<Order>) {
    type Output = Self;

    #[inline]
    fn order(mut self, order: Option<Order>) -> Self::Output {
        self.1 = order;
        self
    }
}

impl Orderable for Column {
    type Output = (Column, Option<Order>);

    #[inline]
    fn order(self, order: Option<Order>) -> Self::Output {
        (self, order)
    }
}

impl Orderable for String {
    type Output = (String, Option<Order>);

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
    use prelude::*;

    #[test]
    fn ascending() {
        let clause = order_by("foo".ascending());
        assert_eq!(clause.compile().unwrap(), "ORDER BY foo ASC");

        let clause = order_by(column("foo").ascending());
        assert_eq!(clause.compile().unwrap(), "ORDER BY `foo` ASC");
    }

    #[test]
    fn descending() {
        let clause = order_by("foo".descending());
        assert_eq!(clause.compile().unwrap(), "ORDER BY foo DESC");

        let clause = order_by(column("foo").descending());
        assert_eq!(clause.compile().unwrap(), "ORDER BY `foo` DESC");
    }

    #[test]
    fn unspecified() {
        let clause = order_by("foo");
        assert_eq!(clause.compile().unwrap(), "ORDER BY foo");

        let clause = order_by(column("foo"));
        assert_eq!(clause.compile().unwrap(), "ORDER BY `foo`");
    }

    #[test]
    fn and() {
        let clause = order_by("foo").and(column("bar").ascending())
                                    .and("baz".to_string().descending());

        assert_eq!(clause.compile().unwrap(), "ORDER BY foo, `bar` ASC, baz DESC");
    }
}
