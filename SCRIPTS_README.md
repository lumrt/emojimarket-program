# 🚀 EmojiMarket Scripts - Quick Start

Scripts pour déployer et interagir avec le programme EmojiMarket sur Solana.

## 📁 Fichiers Créés

```
emojimarket-program/
├── deploy-devnet.sh              # Script de déploiement sur Devnet
├── scripts/
│   ├── client.ts                 # Client TypeScript complet avec PDAs
│   └── example-usage.ts          # Exemples d'utilisation du client
├── config.example.json           # Configuration exemple
└── CLIENT_GUIDE.md               # Guide détaillé du client
```

## ⚡ Quick Start

### 1. Installation

```bash
pnpm install
```

### 2. Déploiement sur Devnet

```bash
# Option 1: Via script npm
pnpm run deploy:devnet

# Option 2: Directement
./deploy-devnet.sh
```

### 3. Utilisation du Client

```bash
# Lancer les exemples
pnpm run example

# Ou utiliser le client directement
pnpm run client
```

## 🎯 Scripts NPM Disponibles

| Commande | Description |
|----------|-------------|
| `pnpm run build` | Build le programme Solana |
| `pnpm run deploy:devnet` | Déploie sur Devnet |
| `pnpm run example` | Lance les exemples d'utilisation |
| `pnpm run client` | Lance le client de base |
| `pnpm run test` | Tests existants |

## 📝 Le Client TypeScript

Le fichier `scripts/client.ts` contient :

### Classes Principales

- **`EmojiMarketClient`** : Client principal pour interagir avec le programme
- **`PDAs`** : Helper pour calculer les Program Derived Addresses

### Méthodes Disponibles

#### Instructions
- `initializeConfig()` - Initialise la configuration
- `createMarket()` - Crée un nouveau market
- `placeBet()` - Place un pari
- `endMarket()` - Termine un market
- `claim()` - Réclame les gains

#### Queries
- `getConfig()` - Récupère la config
- `getMarket()` - Récupère un market
- `getBetAccount()` - Récupère un compte de pari
- `getAllMarkets()` - Récupère tous les markets
- `getUserBets()` - Récupère les paris d'un utilisateur

#### Utils
- `airdrop()` - Demande un airdrop (devnet)
- `getBalance()` - Vérifie le solde
- `displayMarket()` - Affiche un market formaté

### Calcul des PDAs

```typescript
import { PDAs } from "./scripts/client";
import { BN } from "@coral-xyz/anchor";

// Config PDA
const [configPDA, bump] = PDAs.getConfigPDA();

// Market PDA
const [marketPDA, bump] = PDAs.getMarketPDA(new BN(12345));

// BetAccount PDA
const [betPDA, bump] = PDAs.getBetAccountPDA(marketPDA, userPublicKey);

// Market Vault PDA
const [vaultPDA, bump] = PDAs.getMarketVaultPDA(marketPDA);
```

## 💡 Exemple d'Utilisation Rapide

```typescript
import { EmojiMarketClient } from "./scripts/client";
import * as anchor from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

// Setup
const connection = EmojiMarketClient.createConnection("devnet");
const wallet = new anchor.Wallet(Keypair.generate());
const provider = EmojiMarketClient.createProvider(connection, wallet);
const client = new EmojiMarketClient(provider);

// Créer un market
const marketId = new BN(Date.now());
await client.createMarket(
  marketId,
  "Daily Emoji Battle! 🎯",
  null,
  3600 // 1 heure
);

// Parier sur un emoji
await client.placeBet(
  marketId,
  0x1F525, // 🔥
  new BN(5)  // 5 votes
);

// Récupérer le market
const market = await client.getMarket(marketId);
client.displayMarket(market);
```

## 🎨 Emojis Populaires et leurs IDs

```typescript
const EMOJIS = {
  FIRE: 0x1F525,       // 🔥
  ROCKET: 0x1F680,     // 🚀
  GEM: 0x1F48E,        // 💎
  LIGHTNING: 0x26A1,   // ⚡
  MOON: 0x1F319,       // 🌙
  HEART: 0x2764,       // ❤️
  THUMBS_UP: 0x1F44D,  // 👍
  MONEY_BAG: 0x1F4B0,  // 💰
};
```

## 🔧 Configuration

Copiez `config.example.json` vers `config.json` et modifiez selon vos besoins :

```json
{
  "cluster": "devnet",
  "programId": "YOUR_PROGRAM_ID",
  "walletPath": "~/.config/solana/id.json",
  "config": {
    "platformFeeBps": 250,
    "creatorFeeBps": 250,
    "basePriceLamports": "1000000",
    ...
  }
}
```

## 🎮 Scénarios d'Exemple

Le fichier `scripts/example-usage.ts` contient 7 scénarios :

1. **Initialize Config** - Configuration initiale (admin)
2. **Create Market** - Créer un nouveau market
3. **Place Bets** - Placer plusieurs paris
4. **End Market** - Terminer un market
5. **Claim Rewards** - Réclamer les gains
6. **Query All Markets** - Lister tous les markets
7. **Query User Bets** - Lister vos paris

Pour activer un scénario, décommentez-le dans la fonction `main()`.

## 📊 Structure d'un Market

```typescript
interface MarketData {
  creator: PublicKey;
  title: string;
  imageUrl: string | null;
  startTs: BN;
  endTs: BN;
  status: number;          // 0 = Active, 1 = Ended
  totalPot: BN;
  totalVotes: BN;
  emojiIds: number[];
  emojiVotes: BN[];
  winner: number | null;
  platformFeeTaken: BN;
  creatorFeeTaken: BN;
  // ... config snapshot
}
```

## 🔐 Sécurité

⚠️ **Important** :
- Ne commitez JAMAIS vos clés privées
- Utilisez des variables d'environnement
- Testez d'abord sur devnet
- Utilisez des wallets hardware en production

## 🐛 Dépannage

### Erreur: "Insufficient funds"
```bash
# Sur devnet
solana airdrop 2
# Ou dans le code
await client.airdrop(wallet.publicKey, 2);
```

### Erreur: "Account does not exist"
- Vérifiez que le program est déployé
- Vérifiez le Program ID dans la config
- Vérifiez que le market existe

### Erreur: "Transaction simulation failed"
```bash
# Dans un terminal séparé, suivez les logs
solana logs
```

## 📚 Documentation Complète

Pour plus de détails, consultez :
- `CLIENT_GUIDE.md` - Guide complet du client
- `scripts/client.ts` - Code source documenté
- `scripts/example-usage.ts` - Exemples détaillés

## 🔗 Resources

- [Solana Docs](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Web3.js](https://solana-labs.github.io/solana-web3.js/)

## ✨ Prochaines Étapes

1. Déployez sur devnet : `pnpm run deploy:devnet`
2. Lancez les exemples : `pnpm run example`
3. Consultez `CLIENT_GUIDE.md` pour plus d'exemples
4. Intégrez dans votre application !

---

**Made with ❤️ for the EmojiMarket community**

