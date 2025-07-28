# 🚀 Production Deployment Guide

## Overview

Your Solana blog can be deployed to production in several ways. This guide covers the most popular hosting options for static web applications.

## 📦 Build for Production

First, build your application for production:

```bash
cd frontend

# Build for web (static files)
dx build --platform web --release

# Or using cargo
cargo build --release --features web
```

This creates a `dist/` directory with static files ready for deployment.

## 🌐 Hosting Options

### Option 1: Vercel (Recommended)

**Pros**: Free tier, automatic deployments, great performance
**Best for**: Personal projects, quick deployment

```bash
# Install Vercel CLI
npm install -g vercel

# Deploy
cd frontend
vercel --prod

# Or connect to GitHub for automatic deployments
vercel --prod --yes
```

**Configuration**: Create `vercel.json` in frontend directory:
```json
{
  "buildCommand": "dx build --platform web --release",
  "outputDirectory": "dist",
  "framework": null,
  "rewrites": [
    { "source": "/(.*)", "destination": "/index.html" }
  ]
}
```

### Option 2: Netlify

**Pros**: Free tier, drag-and-drop deployment, forms support
**Best for**: Simple static sites

```bash
# Install Netlify CLI
npm install -g netlify-cli

# Deploy
cd frontend
netlify deploy --prod --dir=dist
```

**Configuration**: Create `netlify.toml`:
```toml
[build]
  publish = "dist"
  command = "dx build --platform web --release"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

### Option 3: GitHub Pages

**Pros**: Free, integrated with GitHub
**Best for**: Open source projects

```bash
# Create GitHub repository
git remote add origin https://github.com/yourusername/solana-blog.git
git push -u origin main

# Enable GitHub Pages in repository settings
# Set source to GitHub Actions
```

**Create `.github/workflows/deploy.yml`**:
```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
    
    - name: Install Dioxus
      run: cargo install dioxus-cli
    
    - name: Build
      run: |
        cd frontend
        dx build --platform web --release
    
    - name: Deploy
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./frontend/dist
```

### Option 4: AWS S3 + CloudFront

**Pros**: Highly scalable, global CDN
**Best for**: High-traffic applications

```bash
# Install AWS CLI
pip install awscli

# Configure AWS credentials
aws configure

# Create S3 bucket
aws s3 mb s3://your-blog-domain.com

# Upload files
aws s3 sync frontend/dist/ s3://your-blog-domain.com --delete

# Enable static website hosting
aws s3 website s3://your-blog-domain.com --index-document index.html --error-document index.html
```

### Option 5: DigitalOcean App Platform

**Pros**: Simple deployment, good performance
**Best for**: Small to medium projects

1. Connect your GitHub repository
2. Set build command: `cd frontend && dx build --platform web --release`
3. Set output directory: `frontend/dist`
4. Deploy

## 🔧 Environment Configuration

### Update Configuration for Production

Edit `frontend/src/config.rs`:

```rust
// For production, use mainnet
pub const DEVNET_RPC_URL: &str = "https://api.mainnet-beta.solana.com";
pub const PROGRAM_ID: &str = "YOUR_MAINNET_PROGRAM_ID";

// Update with your actual blog pubkey
pub const DEMO_BLOG_PUBKEY: &str = "YOUR_ACTUAL_BLOG_PUBKEY";
```

### Environment Variables

Create `.env` file for production:

```env
# Solana Configuration
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_PROGRAM_ID=YOUR_MAINNET_PROGRAM_ID
ARWEAVE_GATEWAY_URL=https://arweave.net

# Optional: Analytics
GOOGLE_ANALYTICS_ID=G-XXXXXXXXXX
```

## 🚀 Deployment Checklist

### Pre-Deployment
- [ ] Build application: `dx build --platform web --release`
- [ ] Test locally: `dx serve --platform web --port 8080`
- [ ] Update configuration for production
- [ ] Set up domain (optional)
- [ ] Configure SSL certificate

### Post-Deployment
- [ ] Test all pages work correctly
- [ ] Test wallet connection
- [ ] Test admin functionality
- [ ] Check mobile responsiveness
- [ ] Set up monitoring (optional)

## 🔒 Security Considerations

### For Production
1. **HTTPS Only**: Ensure your hosting provider supports HTTPS
2. **CORS Configuration**: Configure CORS headers if needed
3. **Content Security Policy**: Add CSP headers
4. **Environment Variables**: Don't commit sensitive data

### Example Security Headers
```html
<!-- Add to index.html -->
<meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self' 'unsafe-inline' https://cdn.tailwindcss.com; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com;">
```

## 📊 Performance Optimization

### Build Optimizations
```bash
# Enable optimizations
RUSTFLAGS="-C target-cpu=native" dx build --platform web --release

# Or for maximum optimization
RUSTFLAGS="-C target-cpu=native -C link-arg=-s" dx build --platform web --release
```

### CDN Configuration
- Enable gzip compression
- Set appropriate cache headers
- Use a CDN for global distribution

## 🎯 Recommended Deployment Strategy

### For Personal Projects
1. **Start with Vercel** (easiest)
2. **Use GitHub Pages** (if open source)
3. **Consider Netlify** (if you need forms)

### For Production Applications
1. **Use AWS S3 + CloudFront** (scalable)
2. **Consider DigitalOcean App Platform** (simple)
3. **Set up monitoring and analytics**

## 🛠️ Troubleshooting

### Common Issues

**Build fails:**
```bash
# Clean and rebuild
cd frontend
cargo clean
dx build --platform web --release
```

**404 errors on refresh:**
- Configure SPA routing (all routes serve index.html)
- Add proper redirects

**Wallet connection issues:**
- Ensure HTTPS is enabled
- Check browser console for errors
- Verify RPC endpoint is accessible

**Performance issues:**
- Enable compression
- Optimize images
- Use CDN for static assets

## 📈 Monitoring and Analytics

### Basic Monitoring
```html
<!-- Add to index.html for basic analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_MEASUREMENT_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_MEASUREMENT_ID');
</script>
```

### Error Tracking
Consider adding error tracking with services like:
- Sentry
- LogRocket
- Bugsnag

## 🎉 Success!

Once deployed, your Solana blog will be:
- ✅ Accessible worldwide
- ✅ Fast and responsive
- ✅ Secure with HTTPS
- ✅ Ready for real blockchain integration

**Your decentralized blog is now live!** 🚀 