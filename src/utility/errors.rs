use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TransformError {
    pub cause: &'static str,
}

impl Error for TransformError {}

impl Display for TransformError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Error in Transform: {}", self.cause)
    }
}

#[derive(Debug)]
pub struct ComparisonError {
    pub cause: &'static str,
}

impl Error for ComparisonError {}

impl Display for ComparisonError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Error in Comparison: {}", self.cause)
    }
}
