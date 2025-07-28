# 🔍 Manual Diagnostic Checklist

Run these commands one by one to diagnose the connection issue:

## Step 1: Check System and Tools

```bash
# Check if you're in the right directory
pwd
ls -la

# Check Rust installation
rustc --version
cargo --version

# Check if Dioxus is installed
which dx
dx --version
```

## Step 2: Check Port Status

```bash
# Check if port 8080 is already in use
lsof -i :8080

# Or on Windows:
netstat -ano | findstr :8080
```

## Step 3: Check Project Structure

```bash
# Check if frontend directory exists
ls -la frontend/

# Check if key files exist
ls -la frontend/Cargo.toml
ls -la frontend/Dioxus.toml
```

## Step 4: Test Compilation

```bash
cd frontend
cargo check --target wasm32-unknown-unknown
```

## Step 5: Try Different Start Methods

### Method 1: Standard Dioxus serve
```bash
cd frontend
dx serve --platform web --port 8080
```

### Method 2: Alternative port
```bash
cd frontend
dx serve --platform web --port 8081
```

### Method 3: Cargo run
```bash
cd frontend
cargo run --features web
```

### Method 4: Desktop mode (fallback)
```bash
cd frontend
cargo run --features desktop
```

## Step 6: Test Connection

Once you start the server, test the connection:

```bash
# In a new terminal, test if server responds
curl -I http://localhost:8080

# Or try the alternative port
curl -I http://localhost:8081
```

## Expected Outputs

### ✅ Successful Server Start:
```
🚀 Starting development server...
📦 Serving static files from: /path/to/frontend/dist
🌐 Dev server running at: http://localhost:8080
```

### ✅ Successful Connection Test:
```
HTTP/1.1 200 OK
content-type: text/html
```

## Common Issues and Solutions

### Issue 1: "dx command not found"
**Solution:**
```bash
cargo install dioxus-cli
```

### Issue 2: "Port 8080 already in use"
**Solution:**
```bash
dx serve --platform web --port 8081
# Then visit http://localhost:8081
```

### Issue 3: Compilation errors
**Solution:**
```bash
cd frontend
cargo clean
cargo build --features web
```

### Issue 4: Server starts but browser can't connect
**Solutions:**
1. Try different browser
2. Clear browser cache
3. Check firewall settings
4. Try `dx serve --platform web --port 8080 --host 0.0.0.0`

## Emergency Fallback

If web server won't work at all:
```bash
cd frontend
cargo run --features desktop
```

This opens a native desktop window with your blog.

---

**Run these commands and let me know what output you get!** 