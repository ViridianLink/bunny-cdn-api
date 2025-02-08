use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BunnyFile {
    pub guid: String,
    pub storage_zone_name: String,
    pub path: String,
    pub object_name: String,
    pub length: u64,
    pub last_changed: String,
    pub server_id: i32,
    pub array_number: i32,
    pub is_directory: bool,
    pub user_id: String,
    pub content_type: String,
    pub date_created: String,
    pub storage_zone_id: i32,
    pub checksum: Option<String>,
    pub replicated_zones: Option<String>,
}

impl BunnyFile {
    pub fn full_path(&self) -> String {
        format!("{}{}", self.path, self.object_name)
    }
}
