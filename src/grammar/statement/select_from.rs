use Result;
use grammar::clause::OrderBy;
use grammar::{Buffer, Clause, Condition, Expression, Statement};

/// A `SELECT FROM` statement.
#[derive(Debug, Default)]
pub struct SelectFrom {
    table: Option<String>,
    columns: Option<Vec<String>>,
    conditions: Option<Vec<Box<Condition>>>,
    order_by: Option<OrderBy>,
    limit: Option<usize>,
}

impl SelectFrom {
    /// Create a `SELECT FROM` statement.
    #[inline]
    pub fn new<T: ToString>(table: T) -> SelectFrom {
        SelectFrom::default().table(table)
    }

    /// Set the table.
    pub fn table<T: ToString>(mut self, value: T) -> Self {
        self.table = Some(value.to_string());
        self
    }

    /// Add a column.
    pub fn column<T: ToString>(mut self, value: T) -> Self {
        push!(self.columns, value.to_string());
        self
    }

    /// Add multiple columns.
    pub fn columns<T: ToString>(mut self, values: &[T]) -> Self {
        for value in values {
            push!(self.columns, value.to_string());
        }
        self
    }

    /// Add a condition.
    pub fn so_that<T>(mut self, value: T) -> Self where T: Condition + 'static {
        push!(self.conditions, Box::new(value));
        self
    }

    /// Add an order.
    pub fn order_by<T>(mut self, value: T) -> Self where T: Expression + 'static {
        self.order_by = Some(match self.order_by.take() {
            Some(order_by) => order_by.append(value),
            _ => OrderBy::default().append(value),
        });
        self
    }

    /// Set the limit.
    pub fn limit(mut self, value: usize) -> Self {
        self.limit = Some(value);
        self
    }
}

impl Statement for SelectFrom {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push("SELECT");
        if let &Some(ref columns) = &self.columns {
            buffer.push({
                let mut buffer = Buffer::new();
                for column in columns {
                    buffer.push(format!("`{}`", column));
                }
                buffer.join(", ")
            });
        } else {
            buffer.push("*");
        }
        buffer.push("FROM");
        buffer.push(format!("`{}`", some!(self.table)));
        if let &Some(ref conditions) = &self.conditions {
            buffer.push("WHERE");
            buffer.push({
                let mut buffer = Buffer::new();
                for condition in conditions {
                    buffer.push(try!(condition.compile()));
                }
                buffer.join(" AND ")
            });
        }
        if let Some(ref order_by) = self.order_by {
            buffer.push(try!(order_by.compile()));
        }
        if let Some(limit) = self.limit {
            buffer.push(format!("LIMIT {}", limit));
        }
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn all() {
        let statement = select_from("foo");
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo`");
    }

    #[test]
    fn columns() {
        let statement = select_from("foo").columns(&["bar", "baz"]);
        assert_eq!(statement.compile().unwrap(), "SELECT `bar`, `baz` FROM `foo`");
    }

    #[test]
    fn like() {
        let statement = select_from("foo").so_that(column("bar").like("%baz%"));
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo` WHERE `bar` LIKE '%baz%'");
    }

    #[test]
    fn order() {
        let statement = select_from("foo").order_by("bar").order_by(column("baz").descending());
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo` ORDER BY bar, `baz` DESC");
    }

    #[test]
    fn limit() {
        let statement = select_from("foo").limit(10);
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo` LIMIT 10");
    }
}
