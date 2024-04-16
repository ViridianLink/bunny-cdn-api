pub mod bunny_file;
pub mod bunny_storage;
mod error;

pub use crate::error::{Error, Result};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
