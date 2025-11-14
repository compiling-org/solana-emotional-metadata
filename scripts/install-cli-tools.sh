#!/bin/bash
# Install CLI tools for all 5 blockchains
# Run this to set up your development environment

echo "============================================"
echo "🛠️  Installing Blockchain CLI Tools"
echo "============================================"

# ============================================
# Node.js/npm (required for most tools)
# ============================================

echo ""
echo "Checking Node.js..."
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found"
    echo "Install from: https://nodejs.org/"
    exit 1
else
    echo "✅ Node.js found: $(node --version)"
fi

# ============================================
# 1. NEAR CLI
# ============================================

echo ""
echo "📦 Installing NEAR CLI..."
if command -v near &> /dev/null; then
    echo "✅ NEAR CLI already installed: $(near --version)"
else
    npm install -g near-cli
    echo "✅ NEAR CLI installed"
fi

echo "To use NEAR:"
echo "  near login                    # Login to testnet"
echo "  near create-account NAME      # Create account"
echo "  near deploy --wasmFile FILE   # Deploy contract"

# ============================================
# 2. Solana CLI
# ============================================

echo ""
echo "📦 Installing Solana CLI..."
if command -v solana &> /dev/null; then
    echo "✅ Solana CLI already installed: $(solana --version)"
else
    echo "Download from: https://docs.solana.com/cli/install-solana-cli-tools"
    echo "  sh -c \"\$(curl -sSfL https://release.solana.com/stable/install)\""
fi

echo "To use Solana:"
echo "  solana config set --url devnet  # Use devnet"
echo "  solana-keygen new                # Create keypair"
echo "  solana airdrop 2                 # Get devnet SOL"

# ============================================
# 3. Anchor (Solana framework)
# ============================================

echo ""
echo "📦 Installing Anchor..."
if command -v anchor &> /dev/null; then
    echo "✅ Anchor already installed: $(anchor --version)"
else
    echo "Installing Anchor (this may take a while)..."
    cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
    echo "✅ Anchor installed"
fi

echo "To use Anchor:"
echo "  anchor init PROJECT    # Create new project"
echo "  anchor build          # Build programs"
echo "  anchor deploy         # Deploy to devnet"

# ============================================
# 4. IPFS
# ============================================

echo ""
echo "📦 Installing IPFS..."
if command -v ipfs &> /dev/null; then
    echo "✅ IPFS already installed: $(ipfs --version)"
else
    echo "Download from: https://docs.ipfs.tech/install/"
    echo "Or use package manager:"
    echo "  macOS: brew install ipfs"
    echo "  Linux: snap install ipfs"
fi

echo "To use IPFS:"
echo "  ipfs init            # Initialize"
echo "  ipfs daemon          # Start daemon"
echo "  ipfs add FILE        # Upload file"

# ============================================
# 5. wasm-pack (for Rust WASM)
# ============================================

echo ""
echo "📦 Installing wasm-pack..."
if command -v wasm-pack &> /dev/null; then
    echo "✅ wasm-pack already installed: $(wasm-pack --version)"
else
    cargo install wasm-pack
    echo "✅ wasm-pack installed"
fi

echo "To use wasm-pack:"
echo "  wasm-pack build --target web   # Build WASM"

# ============================================
# Summary
# ============================================

echo ""
echo "============================================"
echo "✅ CLI Tools Setup Complete!"
echo "============================================"
echo ""
echo "📋 Installed Tools:"
command -v near &> /dev/null && echo "  ✅ NEAR CLI" || echo "  ❌ NEAR CLI"
command -v solana &> /dev/null && echo "  ✅ Solana CLI" || echo "  ❌ Solana CLI"
command -v anchor &> /dev/null && echo "  ✅ Anchor" || echo "  ❌ Anchor"
command -v ipfs &> /dev/null && echo "  ✅ IPFS" || echo "  ❌ IPFS"
command -v wasm-pack &> /dev/null && echo "  ✅ wasm-pack" || echo "  ❌ wasm-pack"
echo ""
echo "🚀 Next steps:"
echo "  1. Run ./deploy-to-testnets.sh for deployment guide"
echo "  2. Open http://localhost:8080/ to test"
echo "  3. Go to Settings tab to connect wallets"
echo ""
