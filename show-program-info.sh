#!/bin/bash

# Script pour afficher les informations du programme

echo "╔════════════════════════════════════════════════════╗"
echo "║         EmojiMarket Program Information           ║"
echo "╚════════════════════════════════════════════════════╝"
echo ""

# Program ID depuis le keypair
if [ -f "target/deploy/emojimarket_program-keypair.json" ]; then
    PROGRAM_ID=$(solana address -k target/deploy/emojimarket_program-keypair.json)
    echo "📝 Program ID: $PROGRAM_ID"
else
    echo "⚠️  Keypair not found. Build the program first with: anchor build"
fi

echo ""
echo "📍 Locations:"
echo "   - Rust code: src/lib.rs (declare_id!)"
echo "   - Anchor config: Anchor.toml"
echo "   - TypeScript client: scripts/client.ts"
echo "   - Keypair: target/deploy/emojimarket_program-keypair.json"
echo ""

# Cluster actuel
CLUSTER=$(solana config get | grep "RPC URL" | awk '{print $3}')
echo "🌐 Current Cluster: $CLUSTER"
echo ""

if [ ! -z "$PROGRAM_ID" ]; then
    echo "🔗 Explorer Links:"
    
    if [[ $CLUSTER == *"devnet"* ]]; then
        echo "   Solana Explorer: https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"
        echo "   SolanaFM: https://solana.fm/address/$PROGRAM_ID?cluster=devnet-alpha"
    elif [[ $CLUSTER == *"mainnet"* ]]; then
        echo "   Solana Explorer: https://explorer.solana.com/address/$PROGRAM_ID"
        echo "   SolanaFM: https://solana.fm/address/$PROGRAM_ID"
    else
        echo "   Solana Explorer: https://explorer.solana.com/address/$PROGRAM_ID?cluster=custom&customUrl=$CLUSTER"
    fi
    echo ""
    
    # Vérifier si le programme existe sur le réseau
    echo "🔍 Checking on-chain status..."
    if solana program show $PROGRAM_ID 2>/dev/null; then
        echo "   ✅ Program is deployed and active"
    else
        echo "   ❌ Program not found on current cluster"
        echo "   💡 Deploy with: pnpm run deploy:devnet"
    fi
fi

echo ""

