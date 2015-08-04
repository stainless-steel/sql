use definition::{Definition, Column};
use statement::Statement;
use {Buffer, Result};

/// A `CREATE TABLE` statement.
#[derive(Clone, Debug, Default)]
pub struct CreateTable {
    columns: Option<Vec<Column>>,
    if_not_exists: Option<()>,
    name: Option<String>,
}

impl CreateTable {
    /// Add a column.
    pub fn column(mut self, column: Column) -> Self {
        let mut columns = self.columns.take().unwrap_or_else(|| vec![]);
        columns.push(column);
        self.columns = Some(columns);
        self
    }

    /// Mark that it should be applied only if the table does not exist.
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = Some(());
        self
    }

    /// Set the name.
    pub fn name<T: ToString>(mut self, value: T) -> Self {
        self.name = Some(value.to_string());
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
    fn compile() {
        let statement = create_table().name("foo")
                                      .if_not_exists()
                                      .column(column().name("bar").kind(Type::Float))
                                      .column(column().name("baz").kind(Type::String));

        assert_eq!(&statement.compile().unwrap(),
                   "CREATE TABLE IF NOT EXISTS `foo` (`bar` REAL, `baz` TEXT)");
    }
}
