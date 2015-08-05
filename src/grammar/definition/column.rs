use grammar::{Definition, Expression};
use {Buffer, Result, Type};

/// A column definition.
#[derive(Clone, Debug, Default)]
pub struct Column {
    name: Option<String>,
    kind: Option<Type>,
    not_null: Option<()>,
}

impl Column {
    /// Create a column definition.
    #[inline]
    pub fn new<T: ToString>(name: T) -> Self {
        Column::default().name(name)
    }

    /// Set the name.
    pub fn name<T: ToString>(mut self, value: T) -> Self {
        self.name = Some(value.to_string());
        self
    }

    /// Set the type.
    pub fn kind(mut self, value: Type) -> Self {
        self.kind = Some(value);
        self
    }

    /// Set the type to `Binary`.
    #[inline]
    pub fn binary(self) -> Self {
        self.kind(Type::Binary)
    }

    /// Set the type to `Float`.
    #[inline]
    pub fn float(self) -> Self {
        self.kind(Type::Float)
    }

    /// Set the type to `Integer`.
    #[inline]
    pub fn integer(self) -> Self {
        self.kind(Type::Integer)
    }

    /// Set the type to `String`.
    #[inline]
    pub fn string(self) -> Self {
        self.kind(Type::String)
    }

    /// Mark that it should not be null.
    pub fn not_null(mut self) -> Self {
        self.not_null = Some(());
        self
    }
}

impl Definition for Column {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push(format!("`{}`", some!(self.name)));
        buffer.push(match some!(self.kind) {
            &Type::Binary => "BLOB",
            &Type::Float => "REAL",
            &Type::Integer => "INTEGER",
            &Type::String => "TEXT",
        });
        if let Some(_) = self.not_null {
            buffer.push("NOT NULL");
        }
        Ok(buffer.join(" "))
    }
}

impl Expression for Column {
    fn compile(&self) -> Result<String> {
        Ok(format!("`{}`", some!(self.name)))
    }
}

#[cfg(test)]
mod tests {
    use grammar::Definition;
    use prelude::*;

    #[test]
    fn not_null() {
        let column = column("foo").kind(Type::Float).not_null();
        assert_eq!(Definition::compile(&column).unwrap(), "`foo` REAL NOT NULL");
    }
}
