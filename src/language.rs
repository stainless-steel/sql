//! The language.

use grammar::definition::Column;
use grammar::statement::{CreateTable, DeleteFrom, InsertInto, SelectFrom};

/// Create a column definition.
#[inline]
pub fn column<T: ToString>(name: T) -> Column {
    Column::new(name)
}

/// Create a `CREATE TABLE` statement.
#[inline]
pub fn create_table<T: ToString>(name: T) -> CreateTable {
    CreateTable::new(name)
}

/// Create a `DELETE FROM` statement.
#[inline]
pub fn delete_from<T: ToString>(table: T) -> DeleteFrom {
    DeleteFrom::new(table)
}

/// Create an `INSERT INTO` statement.
#[inline]
pub fn insert_into<T: ToString>(table: T) -> InsertInto {
    InsertInto::new(table)
}

/// Create a `SELECT FROM` statement.
#[inline]
pub fn select_from<T: ToString>(table: T) -> SelectFrom {
    SelectFrom::new(table)
}
