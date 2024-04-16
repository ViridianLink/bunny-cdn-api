use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BunnyFile {
    pub guid: String,
    pub storage_zone_name: String,
    pub path: String,
    pub object_name: String,
    pub length: u64,
    pub last_changed: String,
    pub is_directory: bool,
    pub server_id: i32,
    pub user_id: String,
    pub date_created: String,
    pub storage_zone_id: i32,
}
