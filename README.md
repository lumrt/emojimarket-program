# Emoji Market - Solana Program

Un marché de prédiction (prediction market) sur Solana où les utilisateurs achètent des "emoji votes" avec SOL. L'émoji avec le plus de votes à la fin du marché gagne, et les gagnants peuvent réclamer une part prorata du pot après frais.

## 🎯 Framework

Ce programme utilise **Anchor Framework** pour le développement sur Solana, offrant:
- Simplification du boilerplate de sérialisation/désérialisation
- Sécurité accrue avec les macros de validation
- Meilleure gestion des PDAs et des comptes
- Tooling et débogage améliorés

## 📁 Structure du projet

```
emojimarket-program/
├── src/
│   ├── lib.rs                    # Point d'entrée du programme
│   ├── state.rs                  # Structures de données (Config, Market, Bet)
│   ├── error.rs                  # Erreurs personnalisées
│   ├── math.rs                   # Calculs mathématiques (prix, malus, quadratic)
│   └── instructions/
│       ├── mod.rs
│       ├── initialize_config.rs  # Initialisation de la config
│       ├── create_market.rs      # Création d'un marché
│       ├── bet.rs                # Placer un pari
│       ├── end_market.rs         # Terminer un marché
│       └── claim.rs              # Réclamer les gains
├── tests/
│   └── integration_test.rs       # Tests d'intégration
├── scripts/
│   ├── test_program.ts           # Script de test TypeScript
│   └── create_post.ts            # Script existant
├── Cargo.toml
├── Anchor.toml
├── build.sh                      # Script de build
└── test.sh                       # Script de test complet
```

## 🏗️ Architecture

### Comptes (PDAs)

#### Config (PDA: `["config"]`)
- `admin_address`: Administrateur de la plateforme
- `platform_fee_bps`: Frais de plateforme en basis points
- `creator_fee_bps`: Frais du créateur du marché
- `base_price_lamports`: Prix de base par vote
- `malus_k_millis`: Facteur de malus temporel
- `quad_a_micros`, `quad_b_micros`: Facteurs quadratiques
- `min_duration_secs`, `max_duration_secs`: Durées min/max

#### Market (PDA: `["market", creator, market_id]`)
- Informations du marché (titre, image, durées)
- État (Active/Ended), pot total, votes totaux
- Listes des emojis et leurs votes
- Gagnant et frais collectés
- Snapshot des paramètres de Config

#### Bet (PDA: `["bet", market, user]`)
- Pari d'un utilisateur sur un marché
- Emojis votés et quantités
- Total dépensé, statut de réclamation

### Instructions

1. **initialize_config**: Crée la configuration globale
2. **create_market**: Crée un nouveau marché de prédiction
3. **bet**: Place un pari sur un emoji (avec calcul de prix dynamique)
4. **end_market**: Termine le marché et distribue les frais
5. **claim**: Permet aux gagnants de réclamer leurs gains

### Formules de prix

Le prix d'un vote évolue selon:
- **Temps écoulé** (malus exponentiel): `malus = exp((k*x)/(1-x)) - 1`
- **Votes existants** (quadratique): `f(n) = 1 + a*n + b*n²`
- **Prix final**: `base_price * (1 + malus) * f(n)`

Tous les calculs utilisent des mathématiques à point fixe (pas de floats).

## 🔧 Installation

### ⚠️ Installation requise

**Avant de pouvoir builder le programme, vous devez installer Solana CLI:**

```bash
# Script d'installation automatique (recommandé)
./install-solana.sh

# Puis recharger le shell
source ~/.bashrc  # ou source ~/.zshrc
```

### Prérequis

1. ✅ **Rust** avec rustup (déjà installé)
2. ⚠️ **Solana CLI tools** (à installer avec `./install-solana.sh`)
3. ❌ **Anchor Framework** (optionnel, non requis pour ce projet)

Pour plus de détails, voir [SETUP.md](SETUP.md)

### Build

```bash
# Méthode 1: Script de build
./build.sh

# Méthode 2: Avec Anchor (si installé)
anchor build

# Méthode 3: Avec cargo directement
cargo build --release --target bpfel-unknown-unknown
```

## 🧪 Tests

### Script automatisé (recommandé)

Le script `test.sh` orchestre tout le processus:

```bash
./test.sh
```

Ce script:
1. Nettoie l'état précédent
2. Build le programme
3. Démarre un validateur local
4. Déploie le programme
5. Crée des tokens de test
6. Exécute les tests Rust et TypeScript

### Tests manuels

```bash
# Démarrer le validateur
solana-test-validator --reset --ledger ./.test-ledger

# Dans un autre terminal
solana config set --url localhost
./build.sh
solana program deploy target/deploy/emojimarket_program.so

# Lancer les tests
npm run test
```

### Arrêter le validateur

```bash
pkill -f solana-test-validator
```

## 📝 Utilisation

### 1. Initialiser la configuration

```rust
initialize_config(
    admin_address,
    platform_fee_bps: 250,    // 2.5%
    creator_fee_bps: 250,     // 2.5%
    base_price_lamports: 1_000_000,  // 0.001 SOL
    malus_k_millis: 2000,     // k=2.0
    quad_a_micros: 100,       // a=0.0001
    quad_b_micros: 10,        // b=0.00001
    min_duration_secs: 60,    // 1 minute
    max_duration_secs: 2_592_000,  // 30 jours
)
```

### 2. Créer un marché

```rust
create_market(
    market_id: 1,
    title: "Meilleur emoji 2025?",
    image_url: Some("https://..."),
    end_ts: now + 86400,  // 24 heures
)
```

### 3. Placer un pari

```rust
bet(
    market_id: 1,
    emoji_id: 128512,  // 😀
    vote_qty: 10,
)
```

### 4. Terminer le marché

```rust
end_market(market_id: 1)
// Appelable par l'admin ou le créateur après end_ts
```

### 5. Réclamer les gains

```rust
claim(market_id: 1)
// Les gagnants reçoivent leur part proportionnelle
```

## 🔐 Sécurité

- Tous les calculs utilisent des entiers ou point fixe (pas de floats)
- Validation stricte des paramètres (durées, frais, etc.)
- PDAs dérivées de manière déterministe
- Snapshot des paramètres par marché (pas d'effet rétroactif)
- Protection contre overflow/underflow

## 📊 Logs

Les logs sont sauvegardés dans `.test-ledger/`:
- `validator.log` - Logs du validateur
- `test_output.log` - Sortie des tests Rust
- `tx_output.log` - Sortie des scripts TypeScript

## 🤝 Contribution

Les contributions sont les bienvenues! Assurez-vous que:
- Le code compile sans warnings
- Les tests passent
- La documentation est à jour

## 📄 Licence

ISC

## 🔗 Liens

- [Anchor Framework](https://www.anchor-lang.com/)
- [Solana Docs](https://docs.solana.com/)
- [Repository](https://github.com/lumrt/emojimarket-program)

