#!/bin/bash
# Build script for Solana Grant - Neuroemotive AI & Stream Diffusion
# Can be run independently from other grants

echo "============================================"
echo "Building Solana Grant Components"
echo "Neuroemotive AI & Stream Diffusion"
echo "============================================"

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "⚠️  Anchor not found. Install with: cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked"
    echo "⚠️  Falling back to regular cargo build..."
    USE_CARGO=true
else
    USE_CARGO=false
fi

echo ""
echo "📦 Building Solana programs..."
cd src/solana-client

if [ "$USE_CARGO" = true ]; then
    cargo build-bpf
    BUILD_STATUS=$?
else
    anchor build
    BUILD_STATUS=$?
fi

if [ $BUILD_STATUS -eq 0 ]; then
    echo "✅ Solana programs built successfully"
    echo "📁 Output: target/deploy/"
else
    echo "❌ Solana program build failed"
    exit 1
fi

cd ../..

# Copy specific documentation to grant repository
echo ""
echo "📄 Copying specific documentation..."
cp SOLANA_SPECIFIC_README.md ../grant-repositories/solana-emotional-metadata/README.md
cp SOLANA_SPECIFIC_TECHNICAL_ARCHITECTURE.md ../grant-repositories/solana-emotional-metadata/TECHNICAL_ARCHITECTURE.md  
cp SOLANA_SPECIFIC_IMPLEMENTATION_REPORT.md ../grant-repositories/solana-emotional-metadata/IMPLEMENTATION_REPORT.md

echo ""
echo "============================================"
echo "✅ Solana Grant Build Complete!"
echo "============================================"
echo ""
echo "Deployment files:"
echo "  - Smart contracts: src/solana-client/target/deploy/"
echo "  - Frontend: test-website/index.html (Neuroemotive AI tab)"
echo ""
echo "To deploy:"
echo "  1. Deploy programs to Solana devnet: anchor deploy"
echo "  2. Update program IDs in test-website/blockchain.js"
echo "  3. Configure Solana wallet in frontend"
echo "  4. Serve test-website/ on web server"
echo ""
