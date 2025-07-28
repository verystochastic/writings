# 🔧 Troubleshooting Connection Issues

## Problem: Browser can't connect to http://localhost:8080

Let's diagnose and fix this step by step:

## 🔍 Step 1: Check if Dioxus is installed

```bash
# Check if dx command exists
which dx

# If not found, install it
cargo install dioxus-cli
```

## 🔍 Step 2: Check if the server is actually running

```bash
# Navigate to frontend directory
cd frontend

# Try to start the server manually
dx serve --platform web --port 8080
```

**Expected output:**
```
🚀 Starting development server...
📦 Serving static files from: /path/to/frontend/dist
🌐 Dev server running at: http://localhost:8080
```

## 🔍 Step 3: Check for port conflicts

```bash
# Check if port 8080 is already in use
lsof -i :8080

# Or on Windows:
netstat -ano | findstr :8080
```

**If port 8080 is busy, try:**
```bash
dx serve --platform web --port 8081
```

## 🔍 Step 4: Alternative start methods

### Method 1: Direct cargo run
```bash
cd frontend
cargo run --features web
```

### Method 2: Build first, then serve
```bash
cd frontend
cargo build --features web
dx serve --platform web --port 8080
```

### Method 3: Desktop mode (for testing)
```bash
cd frontend
cargo run --features desktop
```

## 🔍 Step 5: Check firewall/antivirus

- **Windows**: Check Windows Defender Firewall
- **macOS**: Check System Preferences > Security & Privacy
- **Linux**: Check iptables or ufw

## 🔍 Step 6: Try different browsers

- Chrome/Chromium
- Firefox
- Safari
- Edge

## 🔍 Step 7: Check for compilation errors

```bash
cd frontend
cargo check --target wasm32-unknown-unknown
```

## 🔍 Step 8: Common Solutions

### Solution 1: Clean and rebuild
```bash
cd frontend
cargo clean
cargo build --features web
dx serve --platform web --port 8080
```

### Solution 2: Use different port
```bash
cd frontend
dx serve --platform web --port 3000
# Then visit http://localhost:3000
```

### Solution 3: Check network interface
```bash
# Try binding to all interfaces
dx serve --platform web --port 8080 --host 0.0.0.0
```

### Solution 4: Manual HTML test
```bash
cd frontend
# Check if index.html exists
ls dist/
# If it exists, try serving with a simple HTTP server
python3 -m http.server 8080
# Then visit http://localhost:8080
```

## 🔍 Step 9: Debug Information

Run this to get debug info:
```bash
cd frontend
echo "=== System Info ==="
uname -a
echo "=== Rust Version ==="
rustc --version
echo "=== Cargo Version ==="
cargo --version
echo "=== Dioxus Version ==="
dx --version
echo "=== Port Check ==="
lsof -i :8080 || echo "Port 8080 is free"
echo "=== Directory Contents ==="
ls -la
echo "=== Cargo Check ==="
cargo check --target wasm32-unknown-unknown
```

## 🚨 Emergency Fallback

If nothing works, try the desktop version:
```bash
cd frontend
cargo run --features desktop
```

This will open a native desktop window with your blog.

## 📞 Still Having Issues?

If none of these solutions work, please provide:
1. Your operating system
2. Rust version (`rustc --version`)
3. Dioxus version (`dx --version`)
4. Any error messages you see
5. Output of `cargo check --target wasm32-unknown-unknown` 