use statement::Statement;
use {Buffer, Result};

/// An `INSERT INTO` statement.
#[derive(Clone, Debug, Default)]
pub struct InsertInto {
    table: Option<String>,
    columns: Option<Vec<String>>,
    batch: Option<usize>,
}

impl InsertInto {
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

    /// Extend for inserting multiple rows at once.
    pub fn batch(mut self, value: usize) -> Self {
        self.batch = Some(value);
        self
    }
}

impl Statement for InsertInto {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push("INSERT INTO");
        buffer.push(format!("`{}`", some!(self, table)));
        buffer.push({
            let names = {
                let mut buffer = Buffer::new();
                for column in some!(self, columns) {
                    buffer.push(format!("`{}`", column));
                }
                buffer
            };
            let values = {
                let mut buffer = Buffer::new();
                for _ in 0..names.len() {
                    buffer.push("?");
                }
                let one = format!("({})", buffer.join(", "));
                let mut buffer = Buffer::new();
                for _ in 0..self.batch.unwrap_or(1) {
                    buffer.push(&one);
                }
                buffer
            };
            format!("({}) VALUES {}", names.join(", "), values.join(", "))
        });
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn batch() {
        let statement = insert_into("foo").columns(&["bar", "baz"]).batch(3);

        assert_eq!(statement.compile().unwrap(),
                   "INSERT INTO `foo` (`bar`, `baz`) VALUES (?, ?), (?, ?), (?, ?)");
    }
}
