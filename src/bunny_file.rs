use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BunnyFile {
    guid: String,
    storage_zone_name: String,
    path: String,
    object_name: String,
    length: u64,
    last_changed: String,
    is_directory: bool,
    server_id: i32,
    user_id: String,
    date_created: String,
    storage_zone_id: i32,
}
