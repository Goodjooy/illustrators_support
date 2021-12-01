use std::{error::Error, fmt::Display};

mod max_len_limit;

pub use max_len_limit::*;


#[derive(Debug)]
pub struct SizeError {
    real: usize,
    expect: usize,
}
impl Display for SizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "size limit is {} but get size {}",
            self.expect, self.real
        )
    }
}

impl Error for SizeError {}

