use crate::{
    arweave::ArweaveClient,
    types::{BlogClientError, BlogInfo, CreatePostRequest, PostInfo, Result},
};
use borsh::BorshDeserialize;
use solana_blog_program::{
    instruction::BlogInstruction,
    state::{Blog, BlogPost},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;

pub struct BlogClient {
    rpc_client: RpcClient,
    arweave_client: ArweaveClient,
    program_id: Pubkey,
    payer: Option<Keypair>,
}

impl BlogClient {
    pub fn new(rpc_url: &str, program_id: &str) -> Result<Self> {
        let program_id = Pubkey::from_str(program_id)
            .map_err(|e| BlogClientError {
                message: format!("Invalid program ID: {}", e),
            })?;

        Ok(Self {
            rpc_client: RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed()),
            arweave_client: ArweaveClient::new(),
            program_id,
            payer: None,
        })
    }

    pub fn with_payer(mut self, payer: Keypair) -> Self {
        self.payer = Some(payer);
        self
    }

    pub fn with_arweave_client(mut self, arweave_client: ArweaveClient) -> Self {
        self.arweave_client = arweave_client;
        self
    }

    pub async fn initialize_blog(
        &self,
        authority: &Keypair,
        title: String,
        description: String,
    ) -> Result<Pubkey> {
        let blog_keypair = Keypair::new();
        let blog_pubkey = blog_keypair.pubkey();

        let instruction_data = BlogInstruction::InitializeBlog {
            title: title.clone(),
            description: description.clone(),
        };

        // Manual serialization to avoid borsh version mismatch
        let serialized_data = borsh::to_vec(&instruction_data)
            .map_err(|e| BlogClientError {
                message: format!("Failed to serialize instruction: {}", e),
            })?;

        let instruction = Instruction::new(
            self.program_id,
            &serialized_data,
            vec![
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new(blog_pubkey, true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| BlogClientError {
                message: format!("Failed to get recent blockhash: {}", e),
            })?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&authority.pubkey()),
            &[authority, &blog_keypair],
            recent_blockhash,
        );

        self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| BlogClientError {
                message: format!("Failed to send transaction: {}", e),
            })?;

        Ok(blog_pubkey)
    }

    pub async fn create_post(
        &self,
        author: &Keypair,
        blog_pubkey: Pubkey,
        request: CreatePostRequest,
    ) -> Result<Pubkey> {
        // Upload image to Arweave if provided
        let arweave_hash = if let Some(image_data) = &request.image_data {
            let content_type = request.image_content_type.as_deref().unwrap_or("image/jpeg");
            self.arweave_client.upload_data(image_data, content_type).await?
        } else {
            // Upload text content to Arweave as fallback
            self.arweave_client.upload_text(&request.content).await?
        };

        let post_keypair = Keypair::new();
        let post_pubkey = post_keypair.pubkey();

        let instruction_data = BlogInstruction::CreatePost {
            title: request.title,
            content: request.content,
            arweave_hash,
        };

        // Manual serialization to avoid borsh version mismatch
        let serialized_data = borsh::to_vec(&instruction_data)
            .map_err(|e| BlogClientError {
                message: format!("Failed to serialize instruction: {}", e),
            })?;

        let instruction = Instruction::new(
            self.program_id,
            &serialized_data,
            vec![
                AccountMeta::new(author.pubkey(), true),
                AccountMeta::new(post_pubkey, true),
                AccountMeta::new(blog_pubkey, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| BlogClientError {
                message: format!("Failed to get recent blockhash: {}", e),
            })?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&author.pubkey()),
            &[author, &post_keypair],
            recent_blockhash,
        );

        self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| BlogClientError {
                message: format!("Failed to send transaction: {}", e),
            })?;

        Ok(post_pubkey)
    }

    pub async fn get_blog(&self, blog_pubkey: Pubkey) -> Result<BlogInfo> {
        let account_data = self
            .rpc_client
            .get_account_data(&blog_pubkey)
            .map_err(|e| BlogClientError {
                message: format!("Failed to get blog account: {}", e),
            })?;

        let blog = Blog::try_from_slice(&account_data)
            .map_err(|e| BlogClientError {
                message: format!("Failed to deserialize blog: {}", e),
            })?;

        Ok(BlogInfo {
            pubkey: blog_pubkey,
            authority: blog.authority,
            title: blog.title,
            description: blog.description,
            post_count: blog.post_count,
            created_at: blog.created_at,
        })
    }

    pub async fn get_post(&self, post_pubkey: Pubkey) -> Result<PostInfo> {
        let account_data = self
            .rpc_client
            .get_account_data(&post_pubkey)
            .map_err(|e| BlogClientError {
                message: format!("Failed to get post account: {}", e),
            })?;

        let post = BlogPost::try_from_slice(&account_data)
            .map_err(|e| BlogClientError {
                message: format!("Failed to deserialize post: {}", e),
            })?;

        Ok(PostInfo {
            pubkey: post_pubkey,
            author: post.author,
            blog: post.blog,
            title: post.title,
            content: post.content,
            arweave_hash: post.arweave_hash,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }

    pub async fn get_posts_by_blog(&self, blog_pubkey: Pubkey) -> Result<Vec<PostInfo>> {
        // This is a simplified implementation. In a real app, you'd want to index posts
        // or use a more efficient method to find all posts for a blog
        let program_accounts = self
            .rpc_client
            .get_program_accounts(&self.program_id)
            .map_err(|e| BlogClientError {
                message: format!("Failed to get program accounts: {}", e),
            })?;

        let mut posts = Vec::new();
        for (pubkey, account) in program_accounts {
            if let Ok(post) = BlogPost::try_from_slice(&account.data) {
                if post.blog == blog_pubkey {
                    posts.push(PostInfo {
                        pubkey,
                        author: post.author,
                        blog: post.blog,
                        title: post.title,
                        content: post.content,
                        arweave_hash: post.arweave_hash,
                        created_at: post.created_at,
                        updated_at: post.updated_at,
                    });
                }
            }
        }

        // Sort by creation time (newest first)
        posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(posts)
    }

    pub async fn update_post(
        &self,
        author: &Keypair,
        post_pubkey: Pubkey,
        title: Option<String>,
        content: Option<String>,
        image_data: Option<(Vec<u8>, String)>, // (data, content_type)
    ) -> Result<()> {
        // Upload new image to Arweave if provided
        let arweave_hash = if let Some((image_data, content_type)) = image_data {
            Some(self.arweave_client.upload_data(&image_data, &content_type).await?)
        } else if let Some(ref content) = content {
            // Upload updated content to Arweave
            Some(self.arweave_client.upload_text(content).await?)
        } else {
            None
        };

        let instruction_data = BlogInstruction::UpdatePost {
            title,
            content,
            arweave_hash,
        };

        // Manual serialization to avoid borsh version mismatch
        let serialized_data = borsh::to_vec(&instruction_data)
            .map_err(|e| BlogClientError {
                message: format!("Failed to serialize instruction: {}", e),
            })?;

        let instruction = Instruction::new(
            self.program_id,
            &serialized_data,
            vec![
                AccountMeta::new(author.pubkey(), true),
                AccountMeta::new(post_pubkey, false),
            ],
        );

        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| BlogClientError {
                message: format!("Failed to get recent blockhash: {}", e),
            })?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&author.pubkey()),
            &[author],
            recent_blockhash,
        );

        self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| BlogClientError {
                message: format!("Failed to send transaction: {}", e),
            })?;

        Ok(())
    }

    pub fn get_arweave_url(&self, tx_id: &str) -> String {
        self.arweave_client.get_url(tx_id)
    }
} 