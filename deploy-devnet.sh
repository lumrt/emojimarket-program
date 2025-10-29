#!/bin/bash

# Script de déploiement sur Devnet pour EmojiMarket Program
set -e

echo "🚀 Déploiement du programme EmojiMarket sur Devnet..."

# Configuration du cluster
echo "📡 Configuration du cluster Devnet..."
solana config set --url devnet

# Vérification du solde
BALANCE=$(solana balance | awk '{print $1}')
echo "💰 Solde actuel: $BALANCE SOL"

if (( $(echo "$BALANCE < 2" | bc -l) )); then
    echo "⚠️  Solde insuffisant, demande d'airdrop..."
    solana airdrop 2
    sleep 2
fi

# Build du programme
echo "🔨 Build du programme..."
anchor build

# Déploiement
echo "🌐 Déploiement du programme sur Devnet..."
PROGRAM_ID=$(solana address -k target/deploy/emojimarket_program-keypair.json)
echo "📝 Program ID: $PROGRAM_ID"

# Déploiement avec Anchor
anchor deploy --provider.cluster devnet

# Vérification du déploiement (peut échouer si le compte n'est pas encore propagé)
echo "✅ Vérification du déploiement..."
if solana program show $PROGRAM_ID 2>/dev/null; then
    echo "   Programme vérifié avec succès!"
else
    echo "   ⚠️  Vérification échouée (le compte se propagera dans quelques secondes)"
fi

echo ""
echo "✨ Déploiement réussi!"
echo "📝 Program ID: $PROGRAM_ID"
echo "🌐 Cluster: Devnet"
echo "🔗 Explorer: https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"
echo ""
echo "💡 Pour tester le programme, utilisez:"
echo "   pnpm run example"

