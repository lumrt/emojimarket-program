#!/bin/bash
set -e

# Build script for Solana program
# This script compiles the Anchor program to BPF/SBF format

echo "🔨 Building Solana program..."

# Add Solana to PATH if it exists but not in current PATH
if [ -d "$HOME/.local/share/solana/install/active_release/bin" ]; then
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
fi

# Check if Solana tools are installed
if ! command -v cargo-build-sbf &> /dev/null && ! command -v cargo-build-bpf &> /dev/null; then
    echo ""
    echo "❌ Error: Solana build tools not found!"
    echo ""
    echo "Please install Solana CLI tools first:"
    echo "  ./install-solana.sh"
    echo ""
    echo "Or add Solana to your PATH:"
    echo "  export PATH=\"\$HOME/.local/share/solana/install/active_release/bin:\$PATH\""
    echo ""
    echo "Or reload your shell:"
    echo "  source ~/.bashrc"
    echo ""
    exit 1
fi

# Build the program
if command -v cargo-build-sbf &> /dev/null; then
    echo "Using cargo-build-sbf..."
    cargo build-sbf --manifest-path=Cargo.toml --sbf-out-dir=target/deploy
elif command -v cargo-build-bpf &> /dev/null; then
    echo "Using cargo-build-bpf..."
    cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=target/deploy
else
    echo "❌ No Solana build command found"
    exit 1
fi

echo "✅ Build complete!"
ls -lh target/deploy/*.so 2>/dev/null || echo "⚠️  .so file not found in target/deploy/"

