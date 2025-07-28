#!/bin/bash

# Create Real Blog Script
echo "🚀 Creating Real Blog on Deployed Program"
echo "=========================================="

# Check if we have enough SOL
echo "💰 Current SOL Balance:"
solana balance

if [ $(solana balance | cut -d' ' -f1 | cut -d'.' -f1) -lt 1 ]; then
    echo "⚠️  Warning: You might need more SOL for transactions"
    echo "💡 Request devnet SOL: solana airdrop 2"
fi

# Create a simple blog creation script
cat > /tmp/create_blog.rs << 'EOF'
use solana_blog_client::BlogClient;
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://api.devnet.solana.com";
    let program_id = "BmqUqrFGJA7C9nzv787SrC59PUs2kSUvgdhDru6NQFmQ";
    
    // Use your existing keypair
    let keypair_path = std::env::var("HOME").unwrap() + "/.config/solana/id.json";
    let authority = read_keypair_file(&keypair_path)?;
    
    println!("🔑 Using keypair: {}", authority.pubkey());
    
    let title = "My Awesome Solana Blog";
    let description = "A real decentralized blog deployed on Solana devnet with Arweave storage";
    
    println!("📝 Creating blog: {}", title);
    
    let client = BlogClient::new(rpc_url, program_id)?;
    
    match client.initialize_blog(&authority, title.to_string(), description.to_string()).await {
        Ok(blog_pubkey) => {
            println!("✅ Blog created successfully!");
            println!("📝 Blog Pubkey: {}", blog_pubkey);
            println!();
            println!("🔧 To use this blog in your frontend, update frontend/src/config.rs:");
            println!("pub const DEMO_BLOG_PUBKEY: &str = \"{}\";", blog_pubkey);
        }
        Err(e) => {
            println!("❌ Failed to create blog: {}", e.message);
        }
    }
    
    Ok(())
}
EOF

# Compile and run the blog creation
echo "🔨 Compiling blog creation script..."
cd /tmp
cat > Cargo.toml << 'EOF'
[package]
name = "create_blog"
version = "0.1.0"
edition = "2021"

[dependencies]
solana-blog-client = { path = "/home/verystochastic/Development/website/solana-blog/client" }
solana-sdk = "1.18"
tokio = { version = "1.0", features = ["full"] }
EOF

cargo run --bin create_blog 2>/dev/null || {
    echo "❌ Failed to compile. Let's try a simpler approach..."
    cd /home/verystochastic/Development/website/solana-blog
    echo "💡 You can manually create a blog by running the client code directly"
}

echo "🎉 Script complete!" 