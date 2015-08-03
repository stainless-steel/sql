use expression::Expression;
use statement::Statement;
use {Buffer, Result};

/// A `SELECT` statement.
#[derive(Debug, Default)]
pub struct Select {
    columns: Option<Vec<String>>,
    limit: Option<usize>,
    table: Option<String>,
    whereins: Option<Vec<Box<Expression>>>,
}

impl Select {
    /// Add a column.
    pub fn column<T: ToString>(mut self, value: T) -> Self {
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(value.to_string());
        self.columns = Some(columns);
        self
    }

    /// Add a constraint.
    pub fn wherein<T: 'static + Expression>(mut self, value: T) -> Self {
        let mut whereins = self.whereins.take().unwrap_or_else(|| vec![]);
        whereins.push(Box::new(value));
        self.whereins = Some(whereins);
        self
    }

    /// Set the limit.
    pub fn limit(mut self, value: usize) -> Self {
        self.limit = Some(value);
        self
    }

    /// Set the table.
    pub fn table<T: ToString>(mut self, value: T) -> Self {
        self.table = Some(value.to_string());
        self
    }
}

impl Statement for Select {
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
        if let &Some(ref whereins) = &self.whereins {
            buffer.push("WHERE");
            buffer.push({
                let mut buffer = Buffer::new();
                for wherein in whereins {
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
    use statement::Statement;

    #[test]
    fn compile_all() {
        let statement = select().table("foo");
        assert_eq!(&statement.compile().unwrap(), "SELECT * FROM `foo`");
    }

    #[test]
    fn compile_limit() {
        let statement = select().table("foo").limit(10);
        assert_eq!(&statement.compile().unwrap(), "SELECT * FROM `foo` LIMIT 10");
    }

    #[test]
    fn compile_like() {
        let statement = select().table("foo").wherein(column().name("bar").like("%baz%"));
        assert_eq!(&statement.compile().unwrap(), "SELECT * FROM `foo` WHERE `bar` LIKE '%baz%'");
    }

    #[test]
    fn compile_subset() {
        let statement = select().table("foo").column("bar").column("baz");
        assert_eq!(&statement.compile().unwrap(), "SELECT `bar`, `baz` FROM `foo`");
    }
}
