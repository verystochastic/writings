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

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

echo -e "${BLUE}"
echo "🚀 Setting up Solana Blog Repository"
echo "===================================="
echo -e "${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

print_step "Initializing Git repository..."

# Initialize git if not already done
if [ ! -d ".git" ]; then
    git init
    print_success "Git repository initialized"
else
    print_info "Git repository already exists"
fi

print_step "Adding files to Git..."

# Add all files
git add .

print_step "Making initial commit..."

# Make initial commit
git commit -m "Initial commit: Solana blog with decentralized content" || {
    print_error "Commit failed. Check if there are changes to commit."
    exit 1
}

print_success "Initial commit created"

print_step "Adding remote repository..."

# Add remote repository
git remote add origin https://github.com/verystochastic/writings.git || {
    print_info "Remote already exists or failed to add"
}

# Verify remote
print_info "Verifying remote repository..."
git remote -v

print_step "Pushing to GitHub..."

# Push to main branch
git push -u origin main || {
    print_error "Push failed. You may need to:"
    print_info "1. Check your GitHub credentials"
    print_info "2. Ensure the repository exists at https://github.com/verystochastic/writings"
    print_info "3. Try: git push -u origin main --force"
    exit 1
}

print_success "Code pushed to GitHub successfully!"

echo ""
echo "🎉 Repository Setup Complete!"
echo "============================"
echo ""
print_info "Next steps:"
echo "1. Go to: https://github.com/verystochastic/writings"
echo "2. Settings → Pages"
echo "3. Source: 'Deploy from a branch'"
echo "4. Branch: 'gh-pages'"
echo "5. Folder: '/ (root)'"
echo "6. Click 'Save'"
echo ""
print_info "Your blog will be available at:"
echo "https://verystochastic.github.io/writings/"
echo ""
print_info "The GitHub Actions workflow will automatically:"
echo "- Build your Dioxus app"
echo "- Deploy to GitHub Pages"
echo "- Update on every push to main"
echo ""
print_success "Your decentralized blog is ready to deploy!" 