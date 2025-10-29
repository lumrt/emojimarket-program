# EmojiMarket Program - Guide Client

Ce guide explique comment utiliser les scripts pour déployer et interagir avec le programme EmojiMarket sur Solana.

## 📦 Installation

```bash
pnpm install
```

## 🚀 Déploiement sur Devnet

Pour déployer le programme sur Devnet :

```bash
pnpm run deploy:devnet
```

Ce script va :
1. Configurer Solana CLI sur devnet
2. Vérifier votre solde (et demander un airdrop si nécessaire)
3. Builder le programme avec Anchor
4. Déployer le programme sur devnet
5. Afficher le Program ID

## 💻 Utilisation du Client TypeScript

### Import du Client

```typescript
import { EmojiMarketClient, PDAs } from "./scripts/client";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

// Créer une connexion
const connection = EmojiMarketClient.createConnection("devnet");

// Charger votre wallet
const wallet = new anchor.Wallet(Keypair.fromSecretKey(
  // Votre clé secrète ici
));

// Créer le provider
const provider = EmojiMarketClient.createProvider(connection, wallet);

// Initialiser le client
const client = new EmojiMarketClient(provider);
```

### 1. Initialiser la Configuration

```typescript
await client.initializeConfig({
  adminAddress: wallet.publicKey,
  platformFeeBps: 250,        // 2.5%
  creatorFeeBps: 250,         // 2.5%
  basePriceLamports: new BN(1_000_000), // 0.001 SOL
  malusKMillis: 500,          // 0.5 malus factor
  quadAMicros: new BN(1_000), // Quadratic parameter A
  quadBMicros: new BN(2_000), // Quadratic parameter B
  minDurationSecs: 60,        // 1 minute minimum
  maxDurationSecs: 604800,    // 7 jours maximum
});
```

### 2. Créer un Market

```typescript
const marketId = new BN(Date.now());

await client.createMarket(
  marketId,
  "Which emoji will trend today?",
  "https://example.com/image.png",
  3600 // Durée: 1 heure
);
```

### 3. Placer un Pari

```typescript
await client.placeBet(
  marketId,
  0x1F525, // 🔥 emoji (unicode codepoint)
  new BN(10) // 10 votes
);
```

### 4. Terminer un Market

```typescript
await client.endMarket(marketId);
```

### 5. Réclamer les Gains

```typescript
await client.claim(marketId);
```

## 📊 Récupération de Données

### Obtenir la Configuration

```typescript
const config = await client.getConfig();
console.log(config);
```

### Obtenir un Market

```typescript
const market = await client.getMarket(marketId);
if (market) {
  client.displayMarket(market);
}
```

### Obtenir tous les Markets

```typescript
const markets = await client.getAllMarkets();
console.log(`${markets.length} markets found`);
```

### Obtenir un BetAccount

```typescript
const bet = await client.getBetAccount(marketId);
if (bet) {
  console.log("Total spent:", bet.totalSpent.toString());
  console.log("Claimed:", bet.claimed);
}
```

### Obtenir tous les paris d'un utilisateur

```typescript
const userBets = await client.getUserBets();
console.log(`${userBets.length} bets found`);
```

## 🔑 Calcul des PDAs

Le client expose une classe `PDAs` pour calculer les Program Derived Addresses :

```typescript
import { PDAs } from "./scripts/client";

// Config PDA
const [configPDA, configBump] = PDAs.getConfigPDA();

// Market PDA
const [marketPDA, marketBump] = PDAs.getMarketPDA(marketId);

// BetAccount PDA
const [betAccountPDA, betBump] = PDAs.getBetAccountPDA(
  marketPDA,
  userPubkey
);

// Market Vault PDA
const [vaultPDA, vaultBump] = PDAs.getMarketVaultPDA(marketPDA);
```

## 🛠️ Utilitaires

### Airdrop (Devnet/Testnet uniquement)

```typescript
await client.airdrop(wallet.publicKey, 2); // 2 SOL
```

### Vérifier le Solde

```typescript
const balance = await client.getBalance(wallet.publicKey);
console.log(`Balance: ${balance} SOL`);
```

## 📝 Exemples d'Emojis et leurs IDs

Les IDs des emojis sont leurs codepoints Unicode :

- 🔥 Fire: `0x1F525` (128293)
- 🚀 Rocket: `0x1F680` (128640)
- 💎 Gem: `0x1F48E` (128142)
- ⚡ Lightning: `0x26A1` (9889)
- 🌙 Moon: `0x1F319` (127769)
- ❤️ Heart: `0x2764` (10084)
- 👍 Thumbs Up: `0x1F44D` (128077)
- 💰 Money Bag: `0x1F4B0` (128176)

## 🧪 Exemple Complet

```typescript
import { EmojiMarketClient, PDAs } from "./scripts/client";
import * as anchor from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

async function example() {
  // Setup
  const connection = EmojiMarketClient.createConnection("devnet");
  const wallet = new anchor.Wallet(Keypair.generate());
  const provider = EmojiMarketClient.createProvider(connection, wallet);
  const client = new EmojiMarketClient(provider);

  // Airdrop pour les tests
  await client.airdrop(wallet.publicKey, 2);

  // Créer un market
  const marketId = new BN(Date.now());
  await client.createMarket(
    marketId,
    "Daily Emoji Battle! 🎯",
    null,
    86400 // 24 heures
  );

  // Parier sur 🔥
  await client.placeBet(
    marketId,
    0x1F525,
    new BN(5)
  );

  // Parier sur 🚀
  await client.placeBet(
    marketId,
    0x1F680,
    new BN(3)
  );

  // Vérifier le market
  const market = await client.getMarket(marketId);
  if (market) {
    client.displayMarket(market);
  }

  // Plus tard... terminer le market
  // await client.endMarket(marketId);

  // Réclamer les gains si gagnant
  // await client.claim(marketId);
}

example().catch(console.error);
```

## 🎯 Scripts NPM Disponibles

```bash
# Builder le programme
pnpm run build

# Déployer sur devnet
pnpm run deploy:devnet

# Lancer le client exemple
pnpm run client

# Tests existants
pnpm run test
pnpm run test:program
```

## 🔐 Sécurité

⚠️ **Important** : Ne commitez JAMAIS vos clés privées dans le code source !

Pour la production, utilisez :
- Variables d'environnement
- Fichiers de configuration sécurisés
- Hardware wallets (Ledger, etc.)

## 📚 Ressources

- [Documentation Solana](https://docs.solana.com/)
- [Documentation Anchor](https://www.anchor-lang.com/)
- [Web3.js Documentation](https://solana-labs.github.io/solana-web3.js/)

## 🐛 Dépannage

### "Insufficient funds"
- Demandez un airdrop : `await client.airdrop(wallet.publicKey, 2)`
- Sur devnet: `solana airdrop 2`

### "Account does not exist"
- Vérifiez que le compte a bien été créé
- Vérifiez le Program ID dans le client

### "Transaction simulation failed"
- Vérifiez les paramètres de la transaction
- Consultez les logs : `solana logs` dans un terminal séparé

## 🤝 Support

Pour toute question ou problème, consultez la documentation du projet ou créez une issue sur GitHub.

