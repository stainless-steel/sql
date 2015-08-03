use definition::Definition;
use expression::{self, Expression};
use operation;
use {Result, Type};

/// A column definition.
#[derive(Clone, Debug, Default)]
pub struct Column {
    name: Option<String>,
    kind: Option<Type>,
}

impl Column {
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
}

impl Definition for Column {
    fn compile(&self) -> Result<String> {
        let kind = match some!(self, kind) {
            &Type::Binary => "BLOB",
            &Type::Float => "REAL",
            &Type::Integer => "INTEGER",
            &Type::String => "TEXT",
        };
        Ok(format!("`{}` {}", some!(self, name), kind))
    }
}

impl Expression for Column {
    fn compile(&self) -> Result<String> {
        Ok(format!("`{}`", some!(self, name)))
    }
}

impl operation::Like for Column {
    fn like<A: ToString>(self, value: A) -> expression::Like<Self> {
        expression::Like(self, value.to_string())
    }
}
