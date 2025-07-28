# 🚀 GitHub Pages Deployment Guide

## Overview

This guide will help you deploy your decentralized Solana blog to GitHub Pages while maintaining the decentralized architecture for content (Arweave + Solana).

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  GitHub Pages   │    │  Solana Program  │    │    Arweave      │
│   (Frontend)    │◄──►│  (Native Rust)   │    │   Content       │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

**What's Hosted Where:**
- ✅ **Frontend UI**: GitHub Pages (free, fast, reliable)
- ✅ **Blog Content**: Arweave (permanent, decentralized)
- ✅ **Blog Metadata**: Solana blockchain (decentralized)
- ✅ **Post Content**: Arweave (permanent, decentralized)

## 📋 Setup Steps

### Step 1: Repository Setup

1. **Push your code to GitHub:**
```bash
git add .
git commit -m "Setup GitHub Pages deployment"
git push origin main
```

2. **Enable GitHub Pages:**
   - Go to your repository on GitHub
   - Settings → Pages
   - Source: "Deploy from a branch"
   - Branch: `gh-pages` (will be created by GitHub Actions)
   - Folder: `/ (root)`
   - Click "Save"

### Step 2: Configuration Files

The following files have been configured for you:

**✅ `frontend/Dioxus.toml`:**
```toml
[web.app]
base_path = "/solana-blog/"
```

**✅ `.github/workflows/deploy.yml`:**
- Automatic builds on push
- WASM compilation
- GitHub Pages deployment

**✅ `frontend/index.html`:**
- Base path configured
- SPA routing support

### Step 3: Deploy

**Automatic Deployment (Recommended):**
```bash
# Just push to main branch
git push origin main
```

**Manual Deployment:**
```bash
cd frontend
dx build --platform web --release
# Then manually upload dist/ to GitHub Pages
```

## 🌐 Access Your Blog

Once deployed, your blog will be available at:
```
https://[your-username].github.io/solana-blog/
```

## 🔧 Configuration Details

### Base Path Configuration

The app is configured to work with the `/solana-blog/` base path:

```rust
// In Dioxus.toml
base_path = "/solana-blog/"
```

```html
<!-- In index.html -->
<base href="/solana-blog/">
```

### SPA Routing

GitHub Pages doesn't support server-side routing, so we use client-side routing:

```html
<!-- 404.html for SPA routing -->
<script>
    const path = window.location.pathname;
    const newPath = '/solana-blog/' + (hash ? hash : '#/');
    window.location.replace(newPath);
</script>
```

## 🎯 Decentralized Content Flow

### How It Works:

1. **GitHub Pages serves the UI** (fast, reliable)
2. **UI fetches blog metadata from Solana** (decentralized)
3. **UI loads content from Arweave** (permanent, decentralized)

### Example Flow:

```
User visits: https://username.github.io/solana-blog/
    ↓
GitHub Pages serves Dioxus WASM app
    ↓
App connects to Solana RPC
    ↓
Fetches blog metadata from your program
    ↓
Loads post content from Arweave URLs
    ↓
Displays decentralized content
```

## 🔧 Customization

### Change Repository Name

If your repository is named differently:

1. **Update `frontend/Dioxus.toml`:**
```toml
[web.app]
base_path = "/your-repo-name/"
```

2. **Update `frontend/index.html`:**
```html
<base href="/your-repo-name/">
```

3. **Update `.github/workflows/deploy.yml` 404.html section**

### Custom Domain

To use a custom domain:

1. **Add CNAME file to `frontend/dist/`:**
```
yourdomain.com
```

2. **Update GitHub Pages settings:**
   - Settings → Pages
   - Add custom domain
   - Enable HTTPS

3. **Update base path to root:**
```toml
[web.app]
base_path = "/"
```

## 🚀 Benefits of This Approach

### ✅ Advantages:
- **Free Hosting**: GitHub Pages is completely free
- **Fast Loading**: CDN-backed, global distribution
- **Easy Updates**: Automatic deployment on git push
- **Reliable**: GitHub's infrastructure
- **Decentralized Content**: Posts still on Arweave/Solana
- **Censorship Resistant**: Content can't be taken down

### ⚠️ Considerations:
- **Centralized UI**: GitHub hosts the frontend
- **GitHub Dependency**: UI depends on GitHub
- **No Server Features**: Static hosting only

## 🛠️ Troubleshooting

### Build Fails
```bash
# Check Rust toolchain
rustup target add wasm32-unknown-unknown

# Clean and rebuild
cd frontend
cargo clean
dx build --platform web --release
```

### 404 Errors
- Ensure `base_path` is correct in `Dioxus.toml`
- Check that `index.html` has correct `<base href="">`
- Verify 404.html is created in the workflow

### Routing Issues
- GitHub Pages doesn't support server-side routing
- All routes must be handled client-side
- Use hash routing or 404 redirects

### CORS Issues
- Ensure Solana RPC allows requests from your domain
- Check Arweave gateway CORS settings

## 📊 Monitoring

### GitHub Actions
- Check Actions tab for build status
- View deployment logs for errors

### GitHub Pages
- Settings → Pages shows deployment status
- View deployment history

## 🎉 Success!

Once deployed, you'll have:
- ✅ **Fast, reliable UI hosting** (GitHub Pages)
- ✅ **Decentralized content** (Arweave + Solana)
- ✅ **Automatic deployments** (GitHub Actions)
- ✅ **Free hosting** (GitHub Pages)
- ✅ **Global CDN** (Fast loading worldwide)

**Your decentralized blog is now live on GitHub Pages!** 🚀

## 🔗 Next Steps

1. **Test the deployment**: Visit your GitHub Pages URL
2. **Connect real wallet**: Test admin functionality
3. **Create real posts**: Use your Solana program
4. **Custom domain**: Add your own domain (optional)
5. **Analytics**: Add Google Analytics (optional)

**Your blog combines the best of both worlds: reliable hosting + decentralized content!** 🌐 