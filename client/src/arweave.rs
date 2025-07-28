use reqwest::Client;
use crate::types::{Result, BlogClientError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ArweaveClient {
    client: Client,
    gateway_url: String,
    wallet_key: Option<String>, // JWK wallet key
}

#[derive(Debug, Serialize, Deserialize)]
struct ArweaveResponse {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArweaveTag {
    name: String,
    value: String,
}

impl ArweaveClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            gateway_url: "https://arweave.net".to_string(),
            wallet_key: None,
        }
    }

    pub fn with_wallet_key(mut self, wallet_key: String) -> Self {
        self.wallet_key = Some(wallet_key);
        self
    }

    pub fn with_gateway_url(mut self, gateway_url: String) -> Self {
        self.gateway_url = gateway_url;
        self
    }

    pub async fn upload_data(&self, data: &[u8], content_type: &str) -> Result<String> {
        // For MVP, we'll use Bundlr which is easier for small uploads
        // In production, you'd want to implement proper Arweave transaction signing
        self.upload_via_bundlr(data, content_type).await
    }

    async fn upload_via_bundlr(&self, data: &[u8], content_type: &str) -> Result<String> {
        let bundlr_url = "https://node1.bundlr.network";
        
        // Create form data
        let form = reqwest::multipart::Form::new()
            .part(
                "file",
                reqwest::multipart::Part::bytes(data.to_vec())
                    .mime_str(content_type)
                    .map_err(|e| BlogClientError {
                        message: format!("Failed to create form part: {}", e),
                    })?,
            );

        let response = self
            .client
            .post(&format!("{}/tx", bundlr_url))
            .multipart(form)
            .send()
            .await
            .map_err(|e| BlogClientError {
                message: format!("Failed to upload to Bundlr: {}", e),
            })?;

        if response.status().is_success() {
            let arweave_response: ArweaveResponse = response
                .json()
                .await
                .map_err(|e| BlogClientError {
                    message: format!("Failed to parse Bundlr response: {}", e),
                })?;
            Ok(arweave_response.id)
        } else {
            Err(BlogClientError {
                message: format!("Bundlr upload failed with status: {}", response.status()),
            })
        }
    }

    pub async fn get_data(&self, tx_id: &str) -> Result<Vec<u8>> {
        let url = format!("{}/{}", self.gateway_url, tx_id);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| BlogClientError {
                message: format!("Failed to fetch from Arweave: {}", e),
            })?;

        if response.status().is_success() {
            let data = response
                .bytes()
                .await
                .map_err(|e| BlogClientError {
                    message: format!("Failed to read Arweave response: {}", e),
                })?;
            Ok(data.to_vec())
        } else {
            Err(BlogClientError {
                message: format!("Arweave fetch failed with status: {}", response.status()),
            })
        }
    }

    pub fn get_url(&self, tx_id: &str) -> String {
        format!("{}/{}", self.gateway_url, tx_id)
    }

    // Simple text upload for blog content
    pub async fn upload_text(&self, text: &str) -> Result<String> {
        self.upload_data(text.as_bytes(), "text/plain").await
    }

    // Upload JSON data
    pub async fn upload_json(&self, data: &serde_json::Value) -> Result<String> {
        let json_str = serde_json::to_string(data)
            .map_err(|e| BlogClientError {
                message: format!("Failed to serialize JSON: {}", e),
            })?;
        self.upload_data(json_str.as_bytes(), "application/json").await
    }
} 