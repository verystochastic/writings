#!/bin/bash

echo "🚀 Starting Solana Blog NMVP..."
echo "=================================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this script from the project root directory"
    exit 1
fi

# Check if Dioxus is installed
if ! command -v dx &> /dev/null; then
    echo "📦 Installing Dioxus CLI..."
    cargo install dioxus-cli
fi

# Navigate to frontend and start server
echo "🌐 Starting development server..."
cd frontend

# Check if port 8080 is available
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null ; then
    echo "⚠️  Port 8080 is in use. Trying port 8081..."
    dx serve --platform web --port 8081
else
    dx serve --platform web --port 8080
fi

echo ""
echo "🎉 NMVP is ready!"
echo "📖 Open your browser and navigate to:"
echo "   http://localhost:8080 (or 8081 if port 8080 was busy)"
echo ""
echo "📋 What to test:"
echo "   1. View the home page"
echo "   2. Click 'Blog' to see your blog"
echo "   3. Click on any post to read it"
echo "   4. Navigate back to the blog"
echo ""
echo "🔧 To stop the server: Ctrl+C" 