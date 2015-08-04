use expression::Expression;
use statement::Statement;
use {Buffer, Result};

/// A `SELECT FROM` statement.
#[derive(Debug, Default)]
pub struct SelectFrom {
    table: Option<String>,
    columns: Option<Vec<String>>,
    constraints: Option<Vec<Box<Expression>>>,
    limit: Option<usize>,
}

impl SelectFrom {
    /// Set the table.
    pub fn table<T: ToString>(mut self, value: T) -> Self {
        self.table = Some(value.to_string());
        self
    }

    /// Add a column.
    pub fn column<T: ToString>(mut self, value: T) -> Self {
        match self.columns {
            Some(ref mut columns) => {
                columns.push(value.to_string());
            },
            _ => {
                self.columns = Some(vec![]);
                return self.column(value);
            },
        }
        self
    }

    /// Add multiple columns.
    pub fn columns<T: ToString>(mut self, values: &[T]) -> Self {
        match self.columns {
            Some(ref mut columns) => {
                for value in values {
                    columns.push(value.to_string());
                }
            },
            _ => {
                self.columns = Some(vec![]);
                return self.columns(values);
            },
        }
        self
    }

    /// Add a constraint.
    pub fn wherein<T: 'static + Expression>(mut self, value: T) -> Self {
        match self.constraints {
            Some(ref mut constraints) => {
                constraints.push(Box::new(value));
            },
            _ => {
                self.constraints = Some(vec![]);
                return self.wherein(value);
            },
        }
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
        buffer.push(format!("`{}`", some!(self, table)));
        if let &Some(ref constraints) = &self.constraints {
            buffer.push("WHERE");
            buffer.push({
                let mut buffer = Buffer::new();
                for wherein in constraints {
                    buffer.push(try!(wherein.compile()));
                }
                buffer.join(" AND ")
            });
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
    fn limit() {
        let statement = select_from("foo").limit(10);
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo` LIMIT 10");
    }

    #[test]
    fn like() {
        let statement = select_from("foo").wherein(column("bar").like("%baz%"));
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo` WHERE `bar` LIKE '%baz%'");
    }
}
