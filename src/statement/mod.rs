//! Statements.

use Result;

/// A statement.
pub trait Statement {
    /// Compile the statement.
    fn compile(&self) -> Result<String>;
}

mod create_table;
mod insert_into;
mod select;

pub use self::create_table::CreateTable;
pub use self::insert_into::InsertInto;
pub use self::select::Select;
