//! The `WHERE` clause.

use Result;
use grammar::{Buffer, Clause, Condition};

/// A `WHERE` clause.
#[derive(Debug, Default)]
pub struct Where(Vec<Box<Condition>>);

impl Where {
    #[inline]
    pub fn and<T>(mut self, condition: T) -> Self where T: Condition + 'static {
        self.0.push(Box::new(condition));
        self
    }
}

impl Clause for Where {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        for condition in &self.0 {
            buffer.push(try!(condition.compile()));
        }
        Ok(format!("WHERE {}", buffer.join(" OR ")))
    }
}
