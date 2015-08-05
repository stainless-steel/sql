use Result;
use grammar::definition::Column;
use grammar::{Buffer, Definition, Statement};

/// A `CREATE TABLE` statement.
#[derive(Clone, Debug, Default)]
pub struct CreateTable {
    name: Option<String>,
    if_not_exists: Option<()>,
    columns: Option<Vec<Column>>,
}

impl CreateTable {
    /// Create a `CREATE TABLE` statement.
    #[inline]
    pub fn new<T: ToString>(name: T) -> Self {
        CreateTable::default().name(name)
    }

    /// Set the name.
    pub fn name<T: ToString>(mut self, name: T) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Mark that it should be applied only if the table does not exist.
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = Some(());
        self
    }

    /// Add a column.
    pub fn column(mut self, name: Column) -> Self {
        push!(self.columns, name);
        self
    }

    /// Add multiple columns.
    pub fn columns(mut self, names: &[Column]) -> Self {
        for name in names {
            push!(self.columns, name.clone());
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
        buffer.push(format!("`{}`", some!(self.name)));
        buffer.push({
            let mut buffer = Buffer::new();
            for column in some!(self.columns) {
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
    fn columns() {
        let statement = create_table("foo").columns(&[column("bar").float(),
                                                      column("baz").string()]);

        assert_eq!(statement.compile().unwrap(), "CREATE TABLE `foo` (`bar` REAL, `baz` TEXT)");
    }

    #[test]
    fn if_not_exists() {
        let statement = create_table("foo").if_not_exists().column(column("bar").float());
        assert_eq!(statement.compile().unwrap(), "CREATE TABLE IF NOT EXISTS `foo` (`bar` REAL)");
    }
}
