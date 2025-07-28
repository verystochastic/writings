use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogInfo {
    pub pubkey: Pubkey,
    pub authority: Pubkey,
    pub title: String,
    pub description: String,
    pub post_count: u64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostInfo {
    pub pubkey: Pubkey,
    pub author: Pubkey,
    pub blog: Pubkey,
    pub title: String,
    pub content: String,
    pub arweave_hash: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub image_data: Option<Vec<u8>>, // Optional image to upload to Arweave
    pub image_content_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BlogClientError {
    pub message: String,
}

impl std::fmt::Display for BlogClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BlogClientError {}

pub type Result<T> = std::result::Result<T, BlogClientError>; 