#!/bin/bash

set -e

echo "🔨 Building Solana Blog MVP..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_step() {
    echo -e "${YELLOW}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Build the workspace
print_step "Building Rust workspace..."
cargo build --workspace
print_success "Workspace built successfully"

# Build the Solana program
print_step "Building Solana program..."
cd program
if command -v cargo-build-bpf &> /dev/null; then
    cargo build-bpf
    print_success "Solana program built successfully"
else
    print_error "cargo-build-bpf not found. Please install Solana CLI tools."
    echo "Run: sh -c \"\$(curl -sSfL https://release.solana.com/v1.17.0/install)\""
    exit 1
fi
cd ..

# Build the frontend for web
print_step "Building frontend for web..."
cd frontend
if command -v dioxus &> /dev/null; then
    dioxus build --features web --release
    print_success "Frontend built successfully"
else
    print_error "Dioxus CLI not found. Installing..."
    cargo install dioxus-cli
    dioxus build --features web --release
    print_success "Frontend built successfully"
fi
cd ..

print_success "🎉 Build completed successfully!"
echo ""
echo "Next steps:"
echo "1. Deploy the Solana program: solana program deploy program/target/deploy/solana_blog_program.so"
echo "2. Update the program ID in your client configuration"
echo "3. Serve the frontend from frontend/dist/" 