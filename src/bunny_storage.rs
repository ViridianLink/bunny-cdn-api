use std::collections::HashMap;
use std::path::Path;

use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Body, Client, ClientBuilder, Response};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

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
    client: Client,
    storage_name: String,
    endpoint: String,
}

impl BunnyStorage {
    pub fn new(storage_name: impl Into<String>, api_key: &str, region: &str) -> Result<Self> {
        let endpoint = ENDPOINTS
            .get(region)
            .ok_or_else(|| Error::InvalidRegion(region.to_string()))?;

        let mut headers = HeaderMap::new();
        headers.insert("Accesskey", HeaderValue::from_str(api_key).unwrap());

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(Self {
            client,
            storage_name: storage_name.into(),
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

        let file = File::open(file_path).await.unwrap();
        let file_size = file.metadata().await.unwrap().len();

        let stream = ReaderStream::new(file);

        let body = Body::wrap_stream(stream);

        println!("Uploading to: {}", url);

        let response = self
            .client
            .put(url)
            .header("Content-Length", file_size)
            .body(body)
            .send()
            .await
            .unwrap();

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

        let response = self.client.get(url).send().await.unwrap();

        if response.status().is_success() {
            Ok(response.json().await.unwrap())
        } else {
            Err(Error::ListResponseError(response.text().await.unwrap()))
        }
    }
}
