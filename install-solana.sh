#!/bin/bash

echo "🚀 Installing Solana tools..."

# Check if Solana is already installed
if command -v solana &> /dev/null; then
    echo "✅ Solana is already installed: $(solana --version)"
    CURRENT_VERSION=$(solana --version | awk '{print $2}')
    read -p "Do you want to reinstall? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

SOLANA_VERSION="v1.18.22"
SOLANA_INSTALL_DIR="$HOME/.local/share/solana"

echo "📦 Installing Solana CLI $SOLANA_VERSION..."
echo ""

# Try multiple methods to install
INSTALL_SUCCESS=0

# Method 1: Official installer with curl
echo "Trying Method 1: Official installer with curl..."
if sh -c "$(curl -sSfL https://release.solana.com/$SOLANA_VERSION/install)" 2>/dev/null; then
    INSTALL_SUCCESS=1
    echo "✅ Installation successful with curl"
else
    echo "⚠️  Method 1 failed"
    
    # Method 2: Try with wget
    echo ""
    echo "Trying Method 2: Official installer with wget..."
    if command -v wget &> /dev/null; then
        if sh -c "$(wget -q -O - https://release.solana.com/$SOLANA_VERSION/install)" 2>/dev/null; then
            INSTALL_SUCCESS=1
            echo "✅ Installation successful with wget"
        else
            echo "⚠️  Method 2 failed"
        fi
    else
        echo "⚠️  wget not available"
    fi
fi

# Method 3: Try without SSL verification (less secure but works)
if [ $INSTALL_SUCCESS -eq 0 ]; then
    echo ""
    echo "Trying Method 3: curl without SSL verification..."
    echo "⚠️  Warning: This method is less secure"
    if sh -c "$(curl -sSfLk https://release.solana.com/$SOLANA_VERSION/install)" 2>/dev/null; then
        INSTALL_SUCCESS=1
        echo "✅ Installation successful (insecure mode)"
    else
        echo "⚠️  Method 3 failed"
    fi
fi

# Method 4: Manual installation from archive
if [ $INSTALL_SUCCESS -eq 0 ]; then
    echo ""
    echo "Trying Method 4: Direct download..."
    
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    ARCH=$(uname -m)
    if [ "$ARCH" = "x86_64" ]; then
        ARCH="x86_64"
    elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
        ARCH="aarch64"
    fi
    
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    RELEASE_URL="https://github.com/solana-labs/solana/releases/download/$SOLANA_VERSION/solana-release-${ARCH}-unknown-${OS}-gnu.tar.bz2"
    
    echo "Downloading from: $RELEASE_URL"
    if curl -L -o solana-release.tar.bz2 "$RELEASE_URL" 2>/dev/null || wget -O solana-release.tar.bz2 "$RELEASE_URL" 2>/dev/null; then
        echo "Extracting..."
        mkdir -p "$SOLANA_INSTALL_DIR/install/releases/$SOLANA_VERSION"
        tar -xjf solana-release.tar.bz2 -C "$SOLANA_INSTALL_DIR/install/releases/$SOLANA_VERSION"
        
        # Create active_release symlink
        ln -sf "$SOLANA_INSTALL_DIR/install/releases/$SOLANA_VERSION/solana-release" "$SOLANA_INSTALL_DIR/install/active_release"
        
        INSTALL_SUCCESS=1
        echo "✅ Installation successful (direct download)"
    else
        echo "⚠️  Method 4 failed"
    fi
    
    cd - > /dev/null
    rm -rf "$TEMP_DIR"
fi

# Check if installation was successful
if [ $INSTALL_SUCCESS -eq 0 ]; then
    echo ""
    echo "❌ All installation methods failed!"
    echo ""
    echo "📝 Manual installation steps:"
    echo ""
    echo "1. Check your internet connection"
    echo "2. Try updating CA certificates:"
    echo "   sudo apt-get update && sudo apt-get install ca-certificates"
    echo ""
    echo "3. Or download manually from:"
    echo "   https://github.com/solana-labs/solana/releases/tag/$SOLANA_VERSION"
    echo ""
    echo "4. Then extract to: $SOLANA_INSTALL_DIR"
    echo ""
    exit 1
fi

# Add to PATH for current session
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Add to shell profile
SHELL_PROFILE=""
if [ -f "$HOME/.bashrc" ]; then
    SHELL_PROFILE="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
    SHELL_PROFILE="$HOME/.zshrc"
fi

if [ -n "$SHELL_PROFILE" ]; then
    if ! grep -q "solana/install/active_release/bin" "$SHELL_PROFILE"; then
        echo "" >> "$SHELL_PROFILE"
        echo "# Solana" >> "$SHELL_PROFILE"
        echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> "$SHELL_PROFILE"
        echo "✅ Added Solana to $SHELL_PROFILE"
    else
        echo "✅ Solana already in $SHELL_PROFILE"
    fi
fi

# Verify installation
echo ""
if command -v solana &> /dev/null; then
    echo "✅ Solana installed successfully!"
    echo "Version: $(solana --version 2>/dev/null | head -1)"
else
    echo "⚠️  Solana installed but not in PATH yet"
    echo "   Please run: source $SHELL_PROFILE"
fi

echo ""
echo "📝 Next steps:"
echo "  1. Restart your terminal or run: source $SHELL_PROFILE"
echo "  2. Verify: solana --version"
echo "  3. Configure: solana config set --url localhost"
echo "  4. Build: ./build.sh"
echo ""

