use definition::{Definition, Column};
use statement::Statement;
use {Buffer, Result};

/// A `CREATE TABLE` statement.
#[derive(Clone, Debug, Default)]
pub struct CreateTable {
    name: Option<String>,
    if_not_exists: Option<()>,
    columns: Option<Vec<Column>>,
}

impl CreateTable {
    /// Set the name.
    pub fn name<T: ToString>(mut self, value: T) -> Self {
        self.name = Some(value.to_string());
        self
    }

    /// Mark that it should be applied only if the table does not exist.
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = Some(());
        self
    }

    /// Add a column.
    pub fn column(mut self, value: Column) -> Self {
        match self.columns {
            Some(ref mut columns) => {
                columns.push(value);
            },
            _ => {
                self.columns = Some(vec![]);
                return self.column(value);
            },
        }
        self
    }

    /// Add multiple columns.
    pub fn columns(mut self, values: &[Column]) -> Self {
        match self.columns {
            Some(ref mut columns) => {
                for value in values {
                    columns.push(value.clone());
                }
            },
            _ => {
                self.columns = Some(vec![]);
                return self.columns(values);
            },
        }
        self
    }
}

impl Statement for CreateTable {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push("CREATE TABLE");
        if let Some(_) = self.if_not_exists {
             buffer.push("IF NOT EXISTS");
        }
        buffer.push(format!("`{}`", some!(self, name)));
        buffer.push({
            let mut buffer = Buffer::new();
            for column in some!(self, columns) {
                buffer.push(try!(column.compile()));
            }
            format!("({})", buffer.join(", "))
        });
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn if_not_exists() {
        let statement = create_table().name("foo").if_not_exists().columns(&[
            column().name("bar").kind(Type::Float),
            column().name("baz").kind(Type::String),
        ]);

        assert_eq!(statement.compile().unwrap(),
                   "CREATE TABLE IF NOT EXISTS `foo` (`bar` REAL, `baz` TEXT)");
    }
}
