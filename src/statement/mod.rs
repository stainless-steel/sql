//! Statements.

use Result;

/// A statement.
pub trait Statement {
    /// Compile the statement.
    fn compile(&self) -> Result<String>;
}

mod create_table;
mod insert_into;
mod select_from;

pub use self::create_table::CreateTable;
pub use self::insert_into::InsertInto;
pub use self::select_from::SelectFrom;

/// The language.
pub mod language {
    use std::default::Default;
    use super::{CreateTable, InsertInto, SelectFrom};

    /// Create a `CREATE TABLE` statement.
    #[inline]
    pub fn create_table<T: ToString>(name: T) -> CreateTable {
        CreateTable::default().name(name)
    }

    /// Create an `INSERT INTO` statement.
    #[inline]
    pub fn insert_into<T: ToString>(table: T) -> InsertInto {
        InsertInto::default().table(table)
    }

    /// Create a `SELECT FROM` statement.
    #[inline]
    pub fn select_from<T: ToString>(table: T) -> SelectFrom {
        SelectFrom::default().table(table)
    }
}
