use std::{collections::HashMap, path::Path};

use lazy_static::lazy_static;
use reqwest::{Client, Response};
use tokio::fs::File;

use crate::bunny_file::BunnyFile;
use crate::{Error, Result};

lazy_static! {
    static ref ENDPOINTS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("de", "storage.bunnycdn.com");
        map.insert("uk", "uk.storage.bunnycdn.com");
        map.insert("ny", "ny.storage.bunnycdn.com");
        map.insert("la", "la.storage.bunnycdn.com");
        map.insert("sg", "sg.storage.bunnycdn.com");
        map.insert("se", "se.storage.bunnycdn.com");
        map.insert("br", "br.storage.bunnycdn.com");
        map.insert("jh", "jh.storage.bunnycdn.com");
        map.insert("syd", "syd.storage.bunnycdn.com");
        map
    };
}

pub struct BunnyStorage {
    storage_name: String,
    api_key: String,
    endpoint: String,
}

impl BunnyStorage {
    pub fn new(
        storage_name: impl Into<String>,
        api_key: impl Into<String>,
        region: &str,
    ) -> Result<Self> {
        let endpoint = ENDPOINTS
            .get(region)
            .ok_or_else(|| Error::InvalidRegion(region.to_string()))?;

        Ok(Self {
            storage_name: storage_name.into(),
            api_key: api_key.into(),
            endpoint: endpoint.to_string(),
        })
    }

    pub fn download(&self, file: &str) -> Result<()> {
        unimplemented!("Download not implemented")
    }

    pub async fn upload(
        &self,
        file_path: impl AsRef<Path>,
        storage_path: &str,
    ) -> Result<Response> {
        let url = format!(
            "https://{}/{}/{}",
            self.endpoint, self.storage_name, storage_path
        );

        let file = File::open(file_path).await?;
        let file_size = file.metadata().await?.len();

        println!("Uploading to: {}", url);

        let response = Client::new()
            .put(url)
            .header("Accesskey", &self.api_key)
            .header("Content-Length", file_size)
            .body(file)
            .send()
            .await?;

        Ok(response)
    }

    pub fn delete(&self, file: &str) -> Result<()> {
        unimplemented!("Delete not implemented")
    }

    pub async fn list(&self, storage_path: &str) -> Result<Vec<BunnyFile>> {
        let url = format!(
            "https://{}/{}/{}",
            self.endpoint, self.storage_name, storage_path
        );

        let response = Client::new()
            .get(url)
            .header("Accesskey", &self.api_key)
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
