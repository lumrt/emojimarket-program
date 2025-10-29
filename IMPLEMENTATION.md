# Implementation Guide - Emoji Market Program

## ✅ Fichiers générés

Voici tous les fichiers du programme Solana créés selon les spécifications:

### 📂 Structure principale (src/)

#### **src/lib.rs**
Point d'entrée du programme Anchor avec le module `#[program]` contenant les 5 instructions:
- `initialize_config` - Initialise la configuration globale
- `create_market` - Crée un nouveau marché
- `bet` - Place un pari sur un emoji
- `end_market` - Termine le marché
- `claim` - Réclame les gains

#### **src/state.rs**
Définit les 3 structures de comptes (PDAs):
- `Config` - Configuration globale (admin, frais, paramètres de prix)
- `Market` - État d'un marché (emojis, votes, pot, gagnant)
- `BetAccount` - Pari d'un utilisateur (votes par emoji, montant dépensé)

#### **src/error.rs**
Définit 17 erreurs personnalisées:
- Validation des frais et durées
- États du marché
- Calculs arithmétiques
- Autorisations

#### **src/math.rs**
Implémente les formules mathématiques complexes en point fixe:
- `calculate_malus()` - Malus temporel exponentiel: `exp((k*x)/(1-x)) - 1`
- `calculate_quadratic_uplift()` - Uplift quadratique: `1 + a*n + b*n²`
- `calculate_unit_price()` - Prix unitaire combiné
- `calculate_fee()` - Calcul des frais en basis points
- `calculate_user_share()` - Part d'un utilisateur dans le pot

Utilise l'approximation de Taylor pour l'exponentielle avec précision à 1e-9.

### 📂 Instructions (src/instructions/)

#### **initialize_config.rs**
- Crée le compte Config PDA avec seeds `["config"]`
- Valide: `platform_fee_bps + creator_fee_bps <= 10000`
- Valide: `min_duration_secs < max_duration_secs`
- Valide: `base_price_lamports > 0`

#### **create_market.rs**
- Crée Market PDA avec seeds `["market", creator, market_id]`
- Snapshot tous les paramètres de Config
- Valide la durée du marché
- Initialise les vecteurs vides pour emojis/votes

#### **bet.rs**
- Crée/met à jour Bet PDA avec seeds `["bet", market, user]`
- Calcule le prix dynamique selon:
  - Temps écoulé (malus exponentiel)
  - Votes existants (uplift quadratique)
- Transfère SOL de l'utilisateur au compte Market
- Met à jour les totaux (pot, votes) et les vecteurs emoji

#### **end_market.rs**
- Détermine le gagnant (plus de votes, ou plus petit ID si égalité)
- Calcule les frais plateforme et créateur
- Transfère les frais aux comptes admin et créateur
- Marque le marché comme terminé

#### **claim.rs**
- Vérifie que l'utilisateur a voté pour l'emoji gagnant
- Calcule la part proportionnelle: `payout_pool * user_votes / total_votes`
- Transfère SOL du Market vers l'utilisateur
- Marque le pari comme réclamé

### 📂 Configuration et tests

#### **Cargo.toml**
- Dépendances: `anchor-lang 0.30.1`, `anchor-spl 0.30.1`
- Feature `init-if-needed` activée
- Dev dependencies pour les tests

#### **Anchor.toml**
- Configuration pour localnet
- Program ID placeholder (à remplacer après déploiement)

#### **tests/integration_test.rs**
Tests de base avec structure pour:
- initialize_config
- create_market
- bet
- end_market
- claim
- workflow complet

#### **scripts/test_program.ts**
Script TypeScript pour tester:
- Dérivation des PDAs
- Structures de comptes
- Prêt pour intégration avec @coral-xyz/anchor

### 📂 Utilitaires

#### **build.sh**
Script de build intelligent qui tente:
1. `cargo build-sbf` (Solana moderne)
2. `cargo build-bpf` (Solana ancien)
3. `cargo build --target bpfel-unknown-unknown` (fallback)

#### **test.sh** (mis à jour)
Script de test complet orchestrant:
1. Nettoyage de l'état
2. Build du programme
3. Démarrage validateur local
4. Déploiement
5. Création tokens test
6. Tests Rust et TypeScript

#### **Makefile**
Commandes pratiques: build, test, clean, deploy, check, fmt

#### **package.json** (mis à jour)
Dépendances TypeScript/Anchor ajoutées:
- `@coral-xyz/anchor`
- `@solana/web3.js`
- `ts-node`, `typescript`

#### **README.md**
Documentation complète:
- Architecture du programme
- Structure des comptes
- Formules de prix
- Instructions d'installation
- Guide d'utilisation
- Exemples de code

## 🎯 Conformité avec les spécifications

### ✅ Comptes (PDAs)

| Spécification | Implémenté | Fichier |
|---------------|------------|---------|
| Config avec tous les champs requis | ✅ | state.rs |
| Market avec snapshot de Config | ✅ | state.rs |
| Bet avec emoji_ids et emoji_votes | ✅ | state.rs |
| Seeds PDA correctes | ✅ | instructions/*.rs |

### ✅ Instructions

| Instruction | Validations | Logique | Fichier |
|-------------|-------------|---------|---------|
| initialize_config | ✅ Frais <= 100%, durées valides, prix > 0 | ✅ | initialize_config.rs |
| create_market | ✅ Durée dans range, snapshot Config | ✅ | create_market.rs |
| bet | ✅ Marché actif, qty >= 1, formules de prix | ✅ | bet.rs |
| end_market | ✅ Temps écoulé, autorisation admin/créateur | ✅ | end_market.rs |
| claim | ✅ Marché terminé, votes gagnants, pas déjà réclamé | ✅ | claim.rs |

### ✅ Formules mathématiques

| Formule | Implémentation | Fichier |
|---------|----------------|---------|
| Malus temporel exponentiel | ✅ Taylor series, point fixe 1e-9 | math.rs |
| Uplift quadratique | ✅ a*n + b*n², précision micro | math.rs |
| Prix unitaire combiné | ✅ base * (1+malus) * f(n) | math.rs |
| Calcul de share prorata | ✅ 128-bit intermediates | math.rs |

### ✅ Sécurité

| Aspect | Implémenté |
|--------|------------|
| Pas de floats | ✅ Tout en entiers/point fixe |
| Checked arithmetic | ✅ Tous les calculs vérifiés |
| Overflow protection | ✅ ErrorCode::ArithmeticOverflow |
| PDA verification | ✅ Anchor seeds & bump |
| Authorization checks | ✅ Admin/creator validation |

## 🚀 Utilisation du test.sh

Le script `test.sh` est maintenant compatible avec le programme généré:

```bash
# Rendre exécutable
chmod +x test.sh build.sh

# Lancer tous les tests
./test.sh
```

Le script va:
1. ✅ Nettoyer l'environnement
2. ✅ Builder avec `build.sh` (fallback intelligent)
3. ✅ Démarrer le validateur local
4. ✅ Déployer le programme
5. ✅ Créer les tokens USDC de test
6. ✅ Exécuter les tests (si disponibles)
7. ✅ Afficher les résultats

## 📝 Prochaines étapes

### Pour utiliser le programme:

1. **Installer les dépendances**:
```bash
npm install
cargo check
```

2. **Builder**:
```bash
./build.sh
# ou
make build
```

3. **Tester**:
```bash
./test.sh
# ou
make test
```

4. **Obtenir le Program ID**:
Après le premier build, récupérer l'ID dans `target/deploy/emojimarket_program-keypair.json` et mettre à jour:
- `src/lib.rs` (ligne `declare_id!`)
- `Anchor.toml` (section `[programs.localnet]`)

### Pour déployer en production:

1. Configurer le réseau:
```bash
solana config set --url mainnet-beta
# ou devnet pour les tests
solana config set --url devnet
```

2. Vérifier le wallet:
```bash
solana balance
```

3. Déployer:
```bash
solana program deploy target/deploy/emojimarket_program.so
```

4. Initialiser la config avec l'admin:
```typescript
await program.methods
  .initializeConfig(
    adminPubkey,
    250, // 2.5% platform fee
    250, // 2.5% creator fee
    1_000_000, // 0.001 SOL base price
    2000, // k=2.0
    100, // a=0.0001
    10, // b=0.00001
    60, // 1 min minimum
    2_592_000, // 30 days maximum
  )
  .accounts({ ... })
  .rpc();
```

## 🎉 Résumé

✅ **5 instructions** implémentées avec toutes les validations
✅ **3 structures de comptes** avec tous les champs spécifiés
✅ **Formules mathématiques** complexes en point fixe
✅ **17 erreurs** personnalisées pour gestion fine
✅ **Scripts de build et test** automatisés
✅ **Documentation complète** (README.md)
✅ **Compatible avec test.sh** fourni

Le programme est prêt à être compilé, testé et déployé ! 🚀

