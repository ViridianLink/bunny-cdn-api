pub mod bunny_file;
pub mod bunny_storage;
pub mod error;

pub use bunny_storage::BunnyStorage;
pub use error::Error;
use error::Result;

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download() {
        let storage = BunnyStorage::new("test", "test", "de").unwrap();
        let dest_file = std::path::Path::new("test.txt");
        storage.download("test.txt", dest_file).await;
    }
}
