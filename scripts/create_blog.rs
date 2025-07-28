use solana_blog_client::BlogClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    commitment_config::CommitmentConfig,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Program configuration
    let rpc_url = "https://api.devnet.solana.com";
    let program_id = "BmqUqrFGJA7C9nzv787SrC59PUs2kSUvgdhDru6NQFmQ";
    
    // Create a new keypair for the blog authority (in production, use your wallet)
    let authority = Keypair::new();
    
    println!("🔑 Blog Authority Pubkey: {}", authority.pubkey());
    println!("💰 Note: This authority needs SOL for transactions");
    
    // Get blog details from command line or use defaults
    let args: Vec<String> = env::args().collect();
    let title = args.get(1).cloned().unwrap_or_else(|| "My Solana Blog".to_string());
    let description = args.get(2).cloned().unwrap_or_else(|| "A decentralized blog on Solana with Arweave storage".to_string());
    
    println!("📝 Creating blog:");
    println!("   Title: {}", title);
    println!("   Description: {}", description);
    
    // Create blog client
    let client = BlogClient::new(rpc_url, program_id)?;
    
    // TODO: In production, you'd need to fund the authority account
    // For now, this will fail unless the authority has SOL
    
    match client.initialize_blog(&authority, title, description).await {
        Ok(blog_pubkey) => {
            println!("✅ Blog created successfully!");
            println!("📝 Blog Pubkey: {}", blog_pubkey);
            println!("🔗 Update frontend/src/config.rs with:");
            println!("   pub const DEMO_BLOG_PUBKEY: &str = \"{}\";", blog_pubkey);
        }
        Err(e) => {
            println!("❌ Failed to create blog: {}", e.message);
            println!("💡 Make sure the authority account has SOL for transactions");
        }
    }
    
    Ok(())
} 