use std::fmt;

#[derive(Debug)]
pub enum MatrixError {
    DimensionMismatch,
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch => write!(f, "DimensionMismatch: matrix dimensions mismatch"),
        }
    }
}
