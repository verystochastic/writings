# 🌟 Solana Blog - Decentralized Blogging Platform

A modern, decentralized blogging platform built with Solana blockchain and Arweave permanent storage. This project demonstrates how to build a full-stack Web3 application using Rust, Dioxus, and native Solana programs (without Anchor).

## ✨ Features

- **🔗 Solana Integration**: Native Solana programs for blog and post management
- **🌐 Arweave Storage**: Permanent, decentralized content storage
- **⚡ Fast UI**: Modern web interface built with Dioxus
- **🔐 Wallet Connect**: Support for Solana wallet integration
- **📱 Responsive**: Works on desktop and mobile devices
- **🎨 Modern Design**: Clean, intuitive user interface

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Dioxus Web    │    │  Solana Program  │    │    Arweave      │
│   Frontend      │◄──►│  (Native Rust)   │    │   Storage       │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                        │                        │
        │                        │                        │
        └────────────────────────┼────────────────────────┘
                                 │
                    ┌──────────────────┐
                    │   Blog Client    │
                    │   (Rust Lib)     │
                    └──────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Solana CLI tools
- Node.js (for Bundlr/Arweave integration)

### Installation

1. **Clone the repository:**
```bash
git clone <repository-url>
cd solana-blog
```

2. **Build the project:**
```bash
cargo build
```

3. **Build and deploy the Solana program:**
```bash
cd program
cargo build-bpf
solana program deploy target/deploy/solana_blog_program.so
```

4. **Run the frontend (web):**
```bash
cd frontend
cargo run --features web
```

5. **Or run the frontend (desktop):**
```bash
cd frontend
cargo run --features desktop
```

## 📁 Project Structure

```
solana-blog/
├── program/           # Solana program (smart contract)
│   ├── src/
│   │   ├── lib.rs         # Program entry point
│   │   ├── instruction.rs  # Instruction definitions
│   │   ├── processor.rs    # Business logic
│   │   └── state.rs        # Data structures
│   └── Cargo.toml
├── client/            # Client library for blockchain interaction
│   ├── src/
│   │   ├── lib.rs         # Client library entry point
│   │   ├── blog_client.rs  # Main client functionality
│   │   ├── arweave.rs     # Arweave integration
│   │   └── types.rs       # Shared types
│   └── Cargo.toml
├── frontend/          # Dioxus web/desktop application
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   ├── app.rs         # Main app component
│   │   ├── components/    # Reusable UI components
│   │   ├── pages/         # Page components
│   │   ├── services/      # Business logic services
│   │   └── utils/         # Utility functions
│   └── Cargo.toml
└── Cargo.toml         # Workspace configuration
```

## 🔧 Development

### Program Development

The Solana program is written in native Rust without Anchor. Key components:

- **Blog State**: Stores blog metadata on-chain
- **Post State**: Stores post metadata with Arweave hash
- **Instructions**: Initialize blog, create post, update post

### Frontend Development

The frontend uses Dioxus for a modern React-like experience in Rust:

```bash
# Development server
cd frontend
cargo run --features web

# Build for production
cargo build --release --features web
```

### Testing

```bash
# Test the program
cd program
cargo test

# Test the client
cd client
cargo test

# Test the frontend
cd frontend
cargo test
```

## 🌐 Deployment

### Deploying the Solana Program

1. **Configure Solana CLI:**
```bash
solana config set --url devnet  # or mainnet-beta
solana config set --keypair ~/.config/solana/id.json
```

2. **Deploy the program:**
```bash
cd program
cargo build-bpf
solana program deploy target/deploy/solana_blog_program.so
```

3. **Note the program ID** and update it in your client configuration.

### Deploying the Frontend

The frontend can be deployed to any static hosting service:

```bash
cd frontend
cargo build --release --features web
# Deploy the generated files to your hosting service
```

## 📝 Usage

### Creating a Blog

1. Visit the application
2. Click "Create Your Blog"
3. Enter blog title and description
4. Confirm the transaction with your Solana wallet

### Creating Posts

1. Navigate to your blog
2. Click "New Post"
3. Write your content
4. Publish (content gets stored on Arweave, metadata on Solana)

### Reading Posts

- Posts are automatically loaded from the blockchain
- Content is fetched from Arweave for permanent availability
- All interactions are decentralized and censorship-resistant

## 🔑 Environment Variables

Create a `.env` file in the root directory:

```env
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_PROGRAM_ID=YourProgramIdHere
ARWEAVE_GATEWAY_URL=https://arweave.net
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Solana Foundation for the blockchain infrastructure
- Arweave team for permanent storage solutions
- Dioxus team for the excellent Rust web framework
- The Rust community for amazing tools and libraries

## 🔗 Links

- [Solana Documentation](https://docs.solana.com/)
- [Arweave Documentation](https://docs.arweave.org/)
- [Dioxus Documentation](https://dioxuslabs.com/)

---

**Built with ❤️ using Rust, Solana, and Arweave** # Trigger deployment
