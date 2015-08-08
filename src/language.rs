//! The language.

use grammar::definition::Column;
use grammar::statement::{CreateTable, Delete, Insert, Select};

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

/// Create a `DELETE` statement.
#[inline]
pub fn delete_from<T: ToString>(table: T) -> Delete {
    Delete::new(table)
}

/// Create an `INSERT` statement.
#[inline]
pub fn insert_into<T: ToString>(table: T) -> Insert {
    Insert::new(table)
}

/// Create a `SELECT` statement.
#[inline]
pub fn select_from<T: ToString>(table: T) -> Select {
    Select::new(table)
}
