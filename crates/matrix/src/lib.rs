mod errors;
mod matrix;

pub use errors::MatrixError;
pub use matrix::Matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
