use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Blog {
    pub authority: Pubkey,
    pub title: String,
    pub description: String,
    pub post_count: u64,
    pub created_at: i64,
}

impl Blog {
    pub const MAX_TITLE_LENGTH: usize = 100;
    pub const MAX_DESCRIPTION_LENGTH: usize = 500;
    
    pub fn get_size(title: &str, description: &str) -> usize {
        32 + // authority
        4 + title.len() + // title
        4 + description.len() + // description
        8 + // post_count
        8   // created_at
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct BlogPost {
    pub author: Pubkey,
    pub blog: Pubkey,
    pub title: String,
    pub content: String,
    pub arweave_hash: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl BlogPost {
    pub const MAX_TITLE_LENGTH: usize = 200;
    pub const MAX_CONTENT_LENGTH: usize = 1000;
    pub const MAX_ARWEAVE_HASH_LENGTH: usize = 43; // Standard Arweave hash length
    
    pub fn get_size(title: &str, content: &str, arweave_hash: &str) -> usize {
        32 + // author
        32 + // blog
        4 + title.len() + // title
        4 + content.len() + // content
        4 + arweave_hash.len() + // arweave_hash
        8 + // created_at
        8   // updated_at
    }
} 