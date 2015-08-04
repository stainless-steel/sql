use Result;
use expression::Expression;

/// A `LIKE` expression.
#[derive(Clone, Debug)]
pub struct Like<T: Expression>(pub T, pub String);

impl<T: Expression> Expression for Like<T> {
    fn compile(&self) -> Result<String> {
        Ok(format!("{} LIKE '{}'", try!(self.0.compile()), self.1))
    }
}
