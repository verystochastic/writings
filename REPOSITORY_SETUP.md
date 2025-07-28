# 🚀 Repository Setup Guide

## Setting up your "writings" repository

This guide will help you push your Solana blog code to `https://github.com/verystochastic/writings.git`.

## 📋 Step-by-Step Instructions

### Step 1: Initialize Git Repository

```bash
# Initialize git in your current directory
git init

# Add all files
git add .

# Make initial commit
git commit -m "Initial commit: Solana blog with decentralized content"
```

### Step 2: Connect to Your GitHub Repository

```bash
# Add the remote repository
git remote add origin https://github.com/verystochastic/writings.git

# Verify the remote was added
git remote -v
```

### Step 3: Push to GitHub

```bash
# Push to main branch
git push -u origin main
```

### Step 4: Enable GitHub Pages

1. **Go to your repository**: https://github.com/verystochastic/writings
2. **Navigate to Settings**: Click "Settings" tab
3. **Go to Pages**: Click "Pages" in the left sidebar
4. **Configure Pages**:
   - Source: "Deploy from a branch"
   - Branch: `gh-pages` (will be created by GitHub Actions)
   - Folder: `/ (root)`
   - Click "Save"

### Step 5: Verify Deployment

Your blog will be available at:
```
https://verystochastic.github.io/writings/
```

## 🔧 Configuration Summary

The following files have been configured for your "writings" repository:

### ✅ `frontend/Dioxus.toml`
```toml
[web.app]
base_path = "/writings/"
```

### ✅ `frontend/index.html`
```html
<base href="/writings/">
```

### ✅ `.github/workflows/deploy.yml`
- Automatic builds on push
- WASM compilation
- GitHub Pages deployment
- SPA routing support

## 🎯 What Happens Next

### Automatic Deployment
- Every push to `main` branch triggers a build
- GitHub Actions compiles your Dioxus app
- Deploys to GitHub Pages automatically
- Your blog updates automatically

### Access Your Blog
- **URL**: https://verystochastic.github.io/writings/
- **Content**: Decentralized (Arweave + Solana)
- **UI**: Hosted on GitHub Pages

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  GitHub Pages   │    │  Solana Program  │    │    Arweave      │
│   (Frontend)    │◄──►│  (Native Rust)   │    │   Content       │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

**What's Hosted Where:**
- ✅ **Frontend UI**: GitHub Pages (https://verystochastic.github.io/writings/)
- ✅ **Blog Content**: Arweave (permanent, decentralized)
- ✅ **Blog Metadata**: Solana blockchain (decentralized)

## 🚀 Quick Commands

```bash
# Initialize and push (run these commands)
git init
git add .
git commit -m "Initial commit: Solana blog with decentralized content"
git remote add origin https://github.com/verystochastic/writings.git
git push -u origin main
```

## 🔍 Troubleshooting

### If push fails:
```bash
# Check if remote is correct
git remote -v

# If you need to change remote
git remote set-url origin https://github.com/verystochastic/writings.git

# Force push if needed (be careful!)
git push -u origin main --force
```

### If GitHub Pages doesn't work:
1. Check Settings → Pages
2. Ensure source is set to "Deploy from a branch"
3. Branch should be `gh-pages`
4. Wait a few minutes for first deployment

### If build fails:
1. Check Actions tab in GitHub
2. View build logs for errors
3. Ensure Rust toolchain is installed

## 🎉 Success!

Once completed, you'll have:
- ✅ **Live blog**: https://verystochastic.github.io/writings/
- ✅ **Automatic deployments**: Push to update
- ✅ **Decentralized content**: Arweave + Solana
- ✅ **Free hosting**: GitHub Pages

**Your decentralized blog is ready!** 🚀 