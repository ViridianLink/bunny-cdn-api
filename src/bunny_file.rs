use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BunnyFile {
    #[serde(rename = "Guid")]
    pub guid: String,
    #[serde(rename = "StorageZoneName")]
    pub storage_zone_name: String,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "ObjectName")]
    pub object_name: String,
    #[serde(rename = "Length")]
    pub length: u64,
    #[serde(rename = "LastChanged")]
    pub last_changed: String,
    #[serde(rename = "ServerId")]
    pub server_id: i32,
    #[serde(rename = "ArrayNumber")]
    pub array_number: i32,
    #[serde(rename = "IsDirectory")]
    pub is_directory: bool,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "ContentType")]
    pub content_type: String,
    #[serde(rename = "DateCreated")]
    pub date_created: String,
    #[serde(rename = "StorageZoneId")]
    pub storage_zone_id: i32,
    #[serde(rename = "Checksum")]
    pub checksum: Option<String>,
    #[serde(rename = "ReplicatedZones")]
    pub replicated_zones: Option<String>,
}
