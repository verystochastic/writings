#!/bin/bash

echo "🔍 Diagnosing Solana Blog Connection Issues"
echo "=========================================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this script from the project root directory"
    exit 1
fi

echo ""
echo "📋 System Information:"
echo "======================"
echo "OS: $(uname -s) $(uname -r)"
echo "Architecture: $(uname -m)"
echo "Rust version: $(rustc --version 2>/dev/null || echo 'Rust not found')"
echo "Cargo version: $(cargo --version 2>/dev/null || echo 'Cargo not found')"

echo ""
echo "🔧 Dioxus Installation:"
echo "======================="
if command -v dx &> /dev/null; then
    echo "✅ Dioxus CLI found: $(dx --version 2>/dev/null || echo 'version unknown')"
else
    echo "❌ Dioxus CLI not found"
    echo "Installing Dioxus CLI..."
    cargo install dioxus-cli
fi

echo ""
echo "🌐 Port Status:"
echo "=============="
if lsof -i :8080 &> /dev/null; then
    echo "⚠️  Port 8080 is in use:"
    lsof -i :8080
else
    echo "✅ Port 8080 is available"
fi

echo ""
echo "📁 Project Structure:"
echo "===================="
echo "Frontend directory: $(ls -la frontend/ 2>/dev/null | head -5)"
echo "Cargo.toml exists: $([ -f frontend/Cargo.toml ] && echo '✅' || echo '❌')"
echo "Dioxus.toml exists: $([ -f frontend/Dioxus.toml ] && echo '✅' || echo '❌')"

echo ""
echo "🔨 Compilation Check:"
echo "===================="
cd frontend
if cargo check --target wasm32-unknown-unknown &> /dev/null; then
    echo "✅ Compilation successful"
else
    echo "❌ Compilation failed"
    echo "Running cargo check to see errors..."
    cargo check --target wasm32-unknown-unknown
fi

echo ""
echo "🚀 Testing Server Start:"
echo "======================="
echo "Attempting to start server on port 8080..."
echo "Press Ctrl+C after 5 seconds to stop the test..."

# Try to start server in background and check if it responds
timeout 5s dx serve --platform web --port 8080 &
SERVER_PID=$!

# Wait a moment for server to start
sleep 2

# Test if server is responding
if curl -s http://localhost:8080 &> /dev/null; then
    echo "✅ Server is responding on http://localhost:8080"
else
    echo "❌ Server is not responding on http://localhost:8080"
    echo "Trying alternative port 8081..."
    if curl -s http://localhost:8081 &> /dev/null; then
        echo "✅ Server is responding on http://localhost:8081"
    else
        echo "❌ Server is not responding on alternative port either"
    fi
fi

# Kill the test server
kill $SERVER_PID 2>/dev/null

echo ""
echo "💡 Suggested Solutions:"
echo "======================"
echo "1. Try: cd frontend && dx serve --platform web --port 8080"
echo "2. Try: cd frontend && dx serve --platform web --port 8081"
echo "3. Try: cd frontend && cargo run --features web"
echo "4. Try: cd frontend && cargo run --features desktop"
echo ""
echo "🔧 If still having issues, check:"
echo "- Firewall settings"
echo "- Antivirus software"
echo "- Browser cache"
echo "- Network interface settings" 