pub mod edge_storage;
pub mod error;

pub use crate::error::{Error, Result};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
