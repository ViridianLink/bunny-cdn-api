pub mod bunny_file;
pub mod bunny_storage;
mod error;

pub use bunny_storage::BunnyStorage;
pub use error::{Error, Result};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
