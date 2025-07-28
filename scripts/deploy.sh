#!/bin/bash

# Solana Blog Program Deployment Script
echo "🚀 Solana Blog Program Deployment"
echo "=================================="

# Check Solana configuration
echo "📋 Current Solana Configuration:"
solana config get
echo ""

# Check balance
echo "💰 Current SOL Balance:"
solana balance
echo ""

# Build the program
echo "🔨 Building Solana program..."
cd program
cargo build-sbf
cd ..

# Deploy the program
echo "🚀 Deploying program to devnet..."
PROGRAM_ID=$(solana program deploy target/deploy/solana_blog_program.so | grep "Program Id:" | awk '{print $3}')

if [ $? -eq 0 ]; then
    echo "✅ Program deployed successfully!"
    echo "📝 Program ID: $PROGRAM_ID"
    echo ""
    
    # Show program info
    echo "📊 Program Information:"
    solana program show $PROGRAM_ID
    echo ""
    
    # Update configuration
    echo "⚙️  To use this program in your frontend, update frontend/src/config.rs:"
    echo "pub const PROGRAM_ID: &str = \"$PROGRAM_ID\";"
    echo ""
    
    echo "🎉 Deployment complete!"
    echo "💡 Remember to update the PROGRAM_ID constant in frontend/src/config.rs"
else
    echo "❌ Deployment failed!"
    exit 1
fi 