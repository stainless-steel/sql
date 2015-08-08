use Result;
use grammar::{Buffer, Statement};

/// A `DELETE FROM` statement.
#[derive(Debug, Default)]
pub struct DeleteFrom {
    table: Option<String>,
}

impl DeleteFrom {
    /// Create a `DELETE FROM` statement.
    #[inline]
    pub fn new<T: ToString>(table: T) -> Self {
        DeleteFrom::default().table(table)
    }

    /// Set the table.
    pub fn table<T: ToString>(mut self, name: T) -> Self {
        self.table = Some(name.to_string());
        self
    }
}

impl Statement for DeleteFrom {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push("DELETE FROM");
        buffer.push(format!("`{}`", some!(self.table)));
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn all() {
        let statement = delete_from("foo");
        assert_eq!(statement.compile().unwrap(), "DELETE FROM `foo`");
    }
}
