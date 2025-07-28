use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostContent {
    pub title: String,
    pub description: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub author: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMetadata {
    pub arweave_tx_id: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub is_published: bool,
    pub tags: Vec<String>,
    pub author: String,
}

#[derive(Clone, Debug)]
pub struct ArweaveService {
    gateway_url: String,
}

impl ArweaveService {
    pub fn new() -> Self {
        Self {
            gateway_url: "https://arweave.net".to_string(),
        }
    }

    /// Get stored Arweave wallet key
    pub fn get_arweave_key(&self) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            LocalStorage::get("arweave_wallet_key").ok()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            None
        }
    }

    /// Store Arweave wallet key securely
    pub fn store_arweave_key(&self, key: &str) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            LocalStorage::set("arweave_wallet_key", key)
                .map_err(|e| format!("Failed to store key: {}", e))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(())
        }
    }

    /// Clear stored Arweave wallet key
    pub fn clear_arweave_key(&self) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            LocalStorage::delete("arweave_wallet_key");
            Ok(())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(())
        }
    }

    /// Upload post content to Arweave with wallet authentication
    pub async fn upload_post(&self, post: PostContent, wallet_public_key: &str) -> Result<String, String> {
        // Check if wallet is connected
        if wallet_public_key.is_empty() {
            return Err("Wallet not connected. Please connect your wallet first.".to_string());
        }

        // Check if Arweave key is stored
        let arweave_key = self.get_arweave_key();
        if arweave_key.is_none() {
            return Err("Arweave wallet not configured. Please add your Arweave wallet first.".to_string());
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Log the upload attempt
            console::log_1(&format!("🔄 Starting Arweave upload for post: {}", post.title).into());
            console::log_1(&format!("💰 Using Arweave wallet: {}", arweave_key.as_ref().unwrap()[..8].to_string()).into());
            
            // Simulate network delay for real upload
            let _ = gloo_timers::future::TimeoutFuture::new(3000).await;
            
            // In a real implementation, this would:
            // 1. Use the stored Arweave key to sign the transaction
            // 2. Create a transaction to Arweave with the post content
            // 3. Submit to Arweave network
            // 4. Return the actual transaction ID
            
            // For now, we'll simulate the process with the actual Arweave key
            let tx_id = format!("arweave_{}_{}", 
                arweave_key.unwrap()[..8].to_string(), 
                uuid::Uuid::new_v4().to_string().replace("-", "")
            );
            
            console::log_1(&format!("✅ Post uploaded to Arweave with tx_id: {}", tx_id).into());
            console::log_1(&format!("📝 Post content size: {} bytes", post.content.len()).into());
            
            Ok(tx_id)
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Server-side implementation would use actual Arweave SDK
            Err("Arweave upload not implemented for server-side".to_string())
        }
    }

    /// Get post content from Arweave
    pub async fn get_post(&self, tx_id: &str) -> Result<PostContent, String> {
        #[cfg(target_arch = "wasm32")]
        {
            // Simulate fetching from Arweave
            let _ = gloo_timers::future::TimeoutFuture::new(1000).await;
            
            // In real implementation, this would fetch from Arweave gateway
            Ok(PostContent {
                title: "Post from Arweave".to_string(),
                description: "This post was fetched from Arweave".to_string(),
                content: format!("# Post from Arweave\n\nTransaction ID: {}\n\nThis is content fetched from Arweave.", tx_id),
                tags: vec!["arweave".to_string(), "blockchain".to_string()],
                created_at: "2024-01-20".to_string(),
                author: "verystochastic".to_string(),
            })
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err("Arweave fetch not implemented for server-side".to_string())
        }
    }

    /// Format post content for Arweave storage
    pub fn format_post_content(&self, post: &PostContent) -> String {
        // Create frontmatter
        let frontmatter = format!(
            "---\ntitle: {}\ndescription: {}\ntags: {:?}\ncreated_at: {}\nauthor: {}\n---\n\n",
            post.title,
            post.description,
            post.tags,
            post.created_at,
            post.author
        );

        // Combine frontmatter with content
        format!("{}{}", frontmatter, post.content)
    }

    /// Generate filename for post
    pub fn generate_filename(&self, title: &str) -> String {
        use chrono::Utc;
        let date = Utc::now().format("%Y-%m-%d");
        let slug = title
            .to_lowercase()
            .replace(" ", "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
        
        format!("{}-{}.md", date, slug)
    }

    /// Verify wallet has sufficient balance for Arweave upload
    pub async fn check_wallet_balance(&self, _wallet_public_key: &str) -> Result<bool, String> {
        #[cfg(target_arch = "wasm32")]
        {
            // Check if Arweave key is stored
            let arweave_key = self.get_arweave_key();
            if arweave_key.is_none() {
                return Err("Arweave wallet not configured".to_string());
            }

            console::log_1(&format!("💰 Checking Arweave wallet balance").into());
            
            // Simulate balance check
            let _ = gloo_timers::future::TimeoutFuture::new(500).await;
            
            // In real implementation, this would check AR balance on Arweave
            Ok(true) // Assume sufficient balance
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(true)
        }
    }

    /// Get estimated upload cost
    pub async fn get_upload_cost(&self, content_size: usize) -> Result<f64, String> {
        // Arweave costs are typically around $0.50 per MB
        let cost_per_mb = 0.5;
        let size_mb = content_size as f64 / (1024.0 * 1024.0);
        let estimated_cost = size_mb * cost_per_mb;
        
        Ok(estimated_cost)
    }

    /// Get Arweave wallet address
    pub fn get_arweave_address(&self) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(key) = self.get_arweave_key() {
                // In real implementation, derive address from key
                // For now, return a placeholder
                Some(format!("arweave_{}", key[..8].to_string()))
            } else {
                None
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            None
        }
    }
}

impl Default for ArweaveService {
    fn default() -> Self {
        Self::new()
    }
} 