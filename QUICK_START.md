# 🚀 Quick Start - NMVP

## Your NMVP is Ready!

Your Solana blog is configured with mock data and ready to view. Here's how to get started:

### Option 1: Use the Script (Recommended)
```bash
chmod +x start-nmvp.sh
./start-nmvp.sh
```

### Option 2: Manual Start
```bash
cd frontend
dx serve --platform web --port 8080
```

### Option 3: Alternative Command
```bash
cd frontend
cargo run --features web
```

## 🌐 Access Your Blog

Open your browser and go to: **http://localhost:8080**

## 📖 What You'll See

### Home Page:
- Clean, dark theme design
- Navigation: Blog, About, Admin
- "verystochastic" branding

### Blog Page (Click "Blog"):
- **Title**: "Solana verystochastic"
- **Description**: "Decentralized finance disasters and lessons from the blockchain"
- **2 Example Posts**:
  1. "GMX - verystochastic" - DeFi exploit analysis
  2. "Solana Validator - verystochastic" - Network incident report

### Individual Posts:
- Full post content
- Clean typography
- Navigation back to blog
- Post metadata (date, author)

## ✅ NMVP Success Criteria

You've reached NMVP when you can:
- ✅ View the home page
- ✅ Navigate to the blog
- ✅ See blog title and description
- ✅ View list of example posts
- ✅ Click and read individual posts
- ✅ Navigate back to the blog

## 🎯 Next Steps

After confirming NMVP works:
1. **Test Admin**: Connect your wallet and test admin panel
2. **Create Posts**: Implement real post creation
3. **Deploy**: Connect to real Solana blockchain
4. **Real Content**: Replace mock data with real posts

## 🛠️ Troubleshooting

**Server won't start?**
```bash
cargo install dioxus-cli
```

**Port 8080 busy?**
```bash
dx serve --platform web --port 8081
```

**Compilation errors?**
```bash
cd frontend && cargo clean && cargo build
```

---

**Your NMVP is ready to test!** 🎉 