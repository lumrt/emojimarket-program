# Emoi Market - Smart Contract Solana

## 🎯 Framework: Anchor

Ce programme utilise **Anchor Framework** pour le développement sur Solana, offrant:
- Simplification du boilerplate de sérialisation/désérialisation
- Sécurité accrue avec les macros de validation
- Meilleure gestion des PDAs et des comptes
- Tooling et débogage améliorés

## Description

Emoi Market est un marché de prédiction (prediction market) où les utilisateurs achètent des "emoji votes" avec SOL. L'émoji avec le plus de votes à la fin du marché gagne. Les utilisateurs gagnants peuvent réclamer une part prorata du pot après frais. 

**Important**: Implémentez exactement les comptes, champs et logique métier spécifiés ci-dessous—ne renommez pas, ne supprimez pas et n'inventez pas de nouveaux champs. Tous les calculs doivent utiliser des mathématiques entières ou à point fixe ; pas de floats.

Accounts

Config (PDA "config")

admin_address: Pubkey – admin of the platform

platform_fee_bps: u16 – platform fee in basis points

creator_fee_bps: u16 – market creator fee in basis points

base_price_lamports: u64 – base price per vote

malus_k_millis: u32 – malus factor multiplier (k * 1000)

quad_a_micros: u64 – quadratic factor a in micro units

quad_b_micros: u64 – quadratic factor b in micro units

min_duration_secs: u32 – minimum market duration

max_duration_secs: u32 – maximum market duration

bump: u8

Market (PDA "market", seeds: ["market", creator, market_id_le:u64])

creator: Pubkey

title: String

image_url: Option<String>

start_ts: i64

end_ts: i64

status: u8 – Active / Ended

total_pot: u64

total_votes: u64

emoji_ids: Vec<u32> – list of emoji ids in the market

emoji_votes: Vec<u64> – vote counts per emoji

winner: Option<u32>

platform_fee_taken: u64

creator_fee_taken: u64

Snapshot of pricing/fee params from Config:

base_price_lamports: u64

malus_k_millis: u32

quad_a_micros: u64

quad_b_micros: u64

platform_fee_bps: u16

creator_fee_bps: u16

min_duration_secs: u32

max_duration_secs: u32

bump: u8

Bet (PDA "bet", seeds: ["bet", market, user])

market: Pubkey

user: Pubkey

emoji_ids: Vec<u32> – emoji ids user voted on

emoji_votes: Vec<u64> – votes per emoji for this user

total_spent: u64

claimed: bool

bump: u8

Instructions

initialize_config

Create the Config account

Validate: platform_fee_bps + creator_fee_bps <= 10000

Validate durations in valid range

Validate base_price_lamports > 0

create_market

Inputs: title, image_url?, end_ts, market_id

Set start_ts = now

Validate duration ∈ [min_duration_secs, max_duration_secs]

Snapshot pricing/fee params from Config

Initialize counters, empty emoji lists

Set status = Active

bet

Inputs: market_id, emoji_id, vote_qty >= 1

Require market Active and now < end_ts

Price formula:

Time progress x = (now - start_ts) / (end_ts - start_ts) ∈ [0, 1)

Malus: malus = exp((k * x) / (1 - x)) - 1, where k = malus_k_millis / 1000

Quadratic uplift on total votes n: f(n) = 1 + a*n + b*n^2, with a = quad_a_micros / 1e6, b = quad_b_micros / 1e6

Unit price = base_price_lamports * (1 + malus) * f(n_before)

Total cost = vote_qty * unit price (round up)

Transfer SOL from user to market vault PDA

Update totals and user’s Bet

end_market

Callable by admin or creator when now ≥ end_ts and status = Active

Winner = emoji with highest votes (tie-break: lowest emoji_id)

Compute fees: platform_fee = pot * platform_fee_bps / 10000, creator_fee = pot * creator_fee_bps / 10000

Payout pool = pot − fees

Pay fees, set winner, mark status = Ended

claim

Require market Ended

User has votes on winning emoji and has not yet claimed

Compute user share: user_share = payout_pool * user_winning_votes / total_winning_votes (128-bit intermediates, round down)

Pay user; mark claimed

Important Notes

Snapshot all Config pricing/fee params when creating a market—later Config changes do not affect existing markets.

All calculations must use fixed-point or integer math; no floats.

Fees are split between platform and market creator; remainder goes to winning voters.

Do not modify, remove, or rename any account, field, or instruction.

---

## 🔧 Build & Setup

### Prerequisites
1. Install Rust with `rustup`
2. Install Solana CLI tools
3. Install Anchor Framework:
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### Build
```bash
anchor build
```

## 🧪 Tests Unitaires Locaux

Le projet inclut une batterie complète de tests pour valider le programme en environnement local.

### Exécution automatisée (recommandé)

Le script `test.sh` à la racine du projet orchestre l'ensemble du processus de test:

```bash
../test.sh
```

Ce script effectue automatiquement:
1. ✅ Nettoyage de l'état précédent du validateur
2. ✅ Build du programme Solana BPF
3. ✅ Démarrage d'un validateur local `solana-test-validator`
4. ✅ Déploiement du programme
5. ✅ Création d'un mock USDC/SOL token
6. ✅ Exécution des tests d'intégration Rust (`cargo test-sbf`)
7. ✅ Exécution des scripts TypeScript de test

### Tests avec Anchor

Pour les tests Anchor natifs:
```bash
anchor test
```

### Tests manuels

Si vous souhaitez exécuter les étapes individuellement:

```bash
# 1. Démarrer le validateur local
solana-test-validator --reset --ledger ./.test-ledger

# 2. Configurer Solana CLI sur localnet
solana config set --url localhost

# 3. Build et déployer avec Anchor
anchor build
anchor deploy

# 4. Lancer les tests
anchor test --skip-local-validator
```

### Nettoyage

Pour arrêter le validateur local:
```bash
pkill -f solana-test-validator
```

### Logs

Les logs sont sauvegardés dans:
- `./.test-ledger/validator.log` - Logs du validateur
- `./.test-ledger/test_output.log` - Sortie des tests Rust
- `./.test-ledger/tx_output.log` - Sortie des scripts TypeScript
- `.anchor/program-logs/` - Logs des programmes Anchor
