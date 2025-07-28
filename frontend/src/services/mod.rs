#[cfg(not(target_arch = "wasm32"))]
use solana_blog_client::{BlogInfo, PostInfo, BlogClient, CreatePostRequest};
#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};

// Include Arweave service
pub mod arweave_service;
pub use arweave_service::{ArweaveService, PostContent, PostMetadata};

// Mock types for WASM builds
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogInfo {
    pub pubkey: String,
    pub authority: String,
    pub title: String,
    pub description: String,
    pub post_count: u32,
    pub created_at: i64,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostInfo {
    pub pubkey: String,
    pub blog: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub image_url: Option<String>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct BlogService {
    // Mock service for WASM builds
}

impl BlogService {
    pub fn new() -> Self {
        Self {}
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn create_client(&self) -> Result<BlogClient, String> {
        BlogClient::new("https://api.devnet.solana.com", crate::config::PROGRAM_ID)
            .map_err(|e| format!("Failed to create client: {}", e))
    }

    pub async fn get_blog(&self, blog_pubkey: &str) -> Result<BlogInfo, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let client = self.create_client()?;
            let pubkey = blog_pubkey.parse()
                .map_err(|e| format!("Invalid pubkey: {}", e))?;
            
            client.get_blog(pubkey).await
                .map_err(|e| format!("Failed to get blog: {}", e))
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Mock implementation for WASM
            if blog_pubkey == crate::config::DEMO_BLOG_PUBKEY {
                Ok(BlogInfo {
                    pubkey: blog_pubkey.to_string(),
                    authority: "11111111111111111111111111111114".to_string(),
                    title: "Solana verystochastic".to_string(),
                    description: "Decentralized finance disasters and lessons from the blockchain".to_string(),
                    post_count: 3,
                    created_at: 1699123456,
                })
            } else {
                Err("Blog not found".to_string())
            }
        }
    }

    pub async fn get_posts(&self, blog_pubkey: &str) -> Result<Vec<PostInfo>, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // For now, return empty list as get_posts doesn't exist in client
            // TODO: Implement get_posts in blog client
            Ok(vec![])
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Mock implementation for WASM
            if blog_pubkey == crate::config::DEMO_BLOG_PUBKEY {
                Ok(vec![
                    PostInfo {
                        pubkey: "11111111111111111111111111111115".to_string(),
                        blog: blog_pubkey.to_string(),
                        author: "11111111111111111111111111111116".to_string(),
                        title: "GMX - verystochastic".to_string(),
                        content: "The largest GMX exploit in DeFi history. Over $50M drained from liquidity pools due to a price manipulation attack on Arbitrum.".to_string(),
                        created_at: 1699789012,
                        image_url: Some("/api/placeholder/600/300".to_string()),
                    },
                    PostInfo {
                        pubkey: "11111111111111111111111111111117".to_string(),
                        blog: blog_pubkey.to_string(),
                        author: "11111111111111111111111111111116".to_string(),
                        title: "Solana Validator - verystochastic".to_string(),
                        content: "Major Solana validator cluster went down for 17 hours due to a botched network upgrade.".to_string(),
                        created_at: 1699702345,
                        image_url: Some("/api/placeholder/600/300".to_string()),
                    },
                ])
            } else {
                Ok(vec![])
            }
        }
    }

    pub async fn create_post(&self, _blog_pubkey: &str, _request: CreatePostRequest) -> Result<String, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Real implementation would go here
            Err("Not implemented for desktop".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Mock implementation for WASM
            // Simulate API delay
            let _ = gloo_timers::future::TimeoutFuture::new(2000).await;
            
            // Return a mock post pubkey
            Ok("11111111111111111111111111111119".to_string())
        }
    }

    pub async fn initialize_blog(&self, _title: String, _description: String) -> Result<String, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Real implementation would go here
            Err("Not implemented for desktop".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Mock implementation for WASM
            // Simulate API delay
            let _ = gloo_timers::future::TimeoutFuture::new(2000).await;
            
            // Return a mock blog pubkey
            Ok(crate::config::DEMO_BLOG_PUBKEY.to_string())
        }
    }
}

// Wallet service for handling Solana wallet connections
#[derive(Debug, Clone)]
pub struct WalletService {
    pub connected: bool,
    pub public_key: Option<String>,
}

impl WalletService {
    pub fn new() -> Self {
        Self {
            connected: false,
            public_key: None,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn connect_phantom(&mut self) -> Result<String, String> {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::window;

        let window = window().ok_or("No window object")?;
        
        // Check if Phantom is installed
        let phantom = js_sys::Reflect::get(&window, &JsValue::from_str("solana"))
            .map_err(|_| "Phantom wallet not found. Please install Phantom wallet extension.")?;
        
        if phantom.is_undefined() {
            return Err("Phantom wallet not found. Please install Phantom wallet extension.".to_string());
        }

        // Connect to wallet
        let connect_method = js_sys::Reflect::get(&phantom, &JsValue::from_str("connect"))
            .map_err(|_| "Failed to get connect method")?;
        
        let connect_fn = connect_method.dyn_ref::<js_sys::Function>()
            .ok_or("Connect method is not a function")?;
        
        let promise = connect_fn.call0(&phantom)
            .map_err(|_| "Failed to call connect")?;
        
        let js_promise = js_sys::Promise::from(promise);
        let result = JsFuture::from(js_promise).await
            .map_err(|_| "Connection rejected by user")?;
        
        // Get public key
        let public_key = js_sys::Reflect::get(&result, &JsValue::from_str("publicKey"))
            .map_err(|_| "Failed to get public key")?;
        
        let public_key_str = js_sys::Reflect::get(&public_key, &JsValue::from_str("toString"))
            .and_then(|to_string| {
                let to_string_fn = to_string.dyn_ref::<js_sys::Function>().ok_or("Not a function")?;
                to_string_fn.call0(&public_key)
            })
            .map_err(|_| "Failed to convert public key to string")?
            .as_string()
            .ok_or("Public key is not a string")?;

        self.connected = true;
        self.public_key = Some(public_key_str.clone());
        
        Ok(public_key_str)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect_phantom(&mut self) -> Result<String, String> {
        // Mock for desktop
        self.connected = true;
        self.public_key = Some("11111111111111111111111111111116".to_string());
        Ok("11111111111111111111111111111116".to_string())
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        self.public_key = None;
    }

    pub fn is_admin(&self) -> bool {
        // Check if connected wallet is the owner/creator
        // Only the original creator wallet gets admin access
        if let Some(public_key) = &self.public_key {
            // Updated with your actual wallet address
            const OWNER_WALLET: &str = "FzdG9aXQN9fZpDyZvbqMu2zG1PzmdyLzX6nQnDRQRZL7";
            
            // For development, you can temporarily log the connected wallet
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("Connected wallet: {}", public_key).into());
            
            public_key == OWNER_WALLET && self.connected
        } else {
            false
        }
    }
} 