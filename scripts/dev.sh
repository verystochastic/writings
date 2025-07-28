#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_step() {
    echo -e "${YELLOW}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

echo -e "${BLUE}"
echo "🌟 Solana Blog - Development Environment"
echo "======================================"
echo -e "${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Check for required tools
print_step "Checking required tools..."

if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

if ! command -v solana &> /dev/null; then
    print_error "Solana CLI not found. Installing..."
    sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
fi

if ! command -v dioxus &> /dev/null; then
    print_info "Dioxus CLI not found. Installing..."
    cargo install dioxus-cli
fi

print_success "All tools available"

# Setup Solana for development
print_step "Setting up Solana for development..."
solana config set --url localhost
solana config set --keypair ~/.config/solana/id.json

# Check if we have a keypair
if [ ! -f ~/.config/solana/id.json ]; then
    print_info "Creating new Solana keypair..."
    solana-keygen new --no-bip39-passphrase
fi

print_success "Solana configuration complete"

# Build the project
print_step "Building project..."
cargo build --workspace
print_success "Project built successfully"

# Ask user what they want to run
echo ""
echo "What would you like to run?"
echo "1) Frontend (web) - Development server"
echo "2) Frontend (desktop) - Native app"
echo "3) Start local Solana validator"
echo "4) Deploy program to local validator"
echo "5) Run all tests"

read -p "Enter your choice (1-5): " choice

case $choice in
    1)
        print_step "Starting frontend development server..."
        cd frontend
        dioxus serve --features web --hot-reload
        ;;
    2)
        print_step "Starting desktop application..."
        cd frontend
        cargo run --features desktop
        ;;
    3)
        print_step "Starting local Solana validator..."
        solana-test-validator
        ;;
    4)
        print_step "Building and deploying program..."
        cd program
        cargo build-bpf
        PROGRAM_ID=$(solana program deploy target/deploy/solana_blog_program.so | grep "Program Id:" | awk '{print $3}')
        print_success "Program deployed with ID: $PROGRAM_ID"
        echo "Update your client configuration with this program ID"
        ;;
    5)
        print_step "Running tests..."
        cargo test --workspace
        print_success "All tests completed"
        ;;
    *)
        print_error "Invalid choice"
        exit 1
        ;;
esac 