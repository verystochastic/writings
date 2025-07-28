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
echo "🚀 Solana Blog - Production Deployment"
echo "====================================="
echo -e "${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Check if Dioxus is installed
if ! command -v dx &> /dev/null; then
    print_error "Dioxus CLI not found. Installing..."
    cargo install dioxus-cli
fi

print_step "Building for production..."

# Navigate to frontend and build
cd frontend

# Clean previous builds
print_info "Cleaning previous builds..."
cargo clean

# Build for production
print_info "Building application..."
dx build --platform web --release

if [ $? -eq 0 ]; then
    print_success "Build completed successfully!"
else
    print_error "Build failed!"
    exit 1
fi

# Check if dist directory was created
if [ ! -d "dist" ]; then
    print_error "dist directory not found. Build may have failed."
    exit 1
fi

print_success "Production build ready in frontend/dist/"

echo ""
echo "🌐 Deployment Options:"
echo "====================="
echo ""
echo "1. Vercel (Recommended for personal projects)"
echo "   npm install -g vercel"
echo "   cd frontend && vercel --prod"
echo ""
echo "2. Netlify (Simple static hosting)"
echo "   npm install -g netlify-cli"
echo "   cd frontend && netlify deploy --prod --dir=dist"
echo ""
echo "3. GitHub Pages (Free for open source)"
echo "   git push origin main"
echo "   Enable GitHub Pages in repository settings"
echo ""
echo "4. Manual upload to any static hosting"
echo "   Upload contents of frontend/dist/ to your hosting provider"
echo ""

print_info "Your static files are ready in: frontend/dist/"
print_info "You can now deploy to any static hosting service."

# Optional: Test the build locally
read -p "Would you like to test the production build locally? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_step "Testing production build..."
    cd dist
    python3 -m http.server 8080 &
    SERVER_PID=$!
    print_success "Server started at http://localhost:8080"
    print_info "Press Ctrl+C to stop the test server"
    wait $SERVER_PID
fi

echo ""
print_success "Deployment script completed!"
print_info "Next steps:"
print_info "1. Choose a hosting provider from the options above"
print_info "2. Upload the contents of frontend/dist/"
print_info "3. Configure your domain (optional)"
print_info "4. Test your live site" 