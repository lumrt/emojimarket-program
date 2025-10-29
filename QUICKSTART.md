# 🚀 Quick Start

## Si c'est votre première utilisation

### Étape 1: Installer Solana (une seule fois)

```bash
./install-solana.sh
```

Puis **redémarrer votre terminal** ou:
```bash
source ~/.bashrc  # bash
source ~/.zshrc   # zsh
```

### Étape 2: Vérifier l'installation

```bash
solana --version
# Devrait afficher: solana-cli 1.18.22
```

### Étape 3: Builder et tester

```bash
./test.sh
```

C'est tout ! 🎉

---

## Commandes utiles

```bash
# Builder uniquement
./build.sh

# Tester uniquement (après build)
npm run test

# Nettoyer
make clean

# Vérifier le code
make check

# Formater le code
make fmt
```

## Structure du projet

```
emojimarket-program/
├── src/                    # Code source Rust
│   ├── lib.rs              # Point d'entrée
│   ├── state.rs            # Structures de données
│   ├── error.rs            # Erreurs
│   ├── math.rs             # Calculs mathématiques
│   └── instructions/       # 5 instructions du programme
├── tests/                  # Tests Rust
├── scripts/                # Scripts TypeScript
├── install-solana.sh       # Installation Solana
├── build.sh                # Build du programme
├── test.sh                 # Tests complets
└── Makefile                # Commandes make
```

## Déploiement

### Local (développement)

```bash
# Le test.sh le fait automatiquement
./test.sh
```

### Devnet (test public)

```bash
# Configurer le cluster
solana config set --url devnet

# Airdrop de SOL
solana airdrop 2

# Builder
./build.sh

# Déployer
solana program deploy target/deploy/emojimarket_program.so
```

### Mainnet (production)

```bash
# Configurer le cluster
solana config set --url mainnet-beta

# Vérifier le solde (vous avez besoin de SOL réel)
solana balance

# Builder
./build.sh

# Déployer (coûte du SOL)
solana program deploy target/deploy/emojimarket_program.so
```

## Problèmes courants

### `command not found: solana`

**Solution**: Installer Solana avec `./install-solana.sh` puis redémarrer le terminal

### `Build failed`

**Solution**: Vérifier que Solana est installé avec `solana --version`

### `Port 8899 already in use`

**Solution**: Arrêter le validateur existant
```bash
pkill -f solana-test-validator
```

### Le build est très lent

**Normal**: La première compilation prend 1-2 minutes. Les suivantes sont plus rapides.

## Documentation complète

- [README.md](README.md) - Documentation principale
- [SETUP.md](SETUP.md) - Guide d'installation détaillé
- [IMPLEMENTATION.md](IMPLEMENTATION.md) - Détails techniques
- [program-instructions.md](program-instructions.md) - Spécifications

## Aide

Besoin d'aide ? Vérifiez:
1. Que Solana est installé: `solana --version`
2. Que le PATH est correct: `which solana`
3. Les logs d'erreur dans `.test-ledger/`

Pour plus d'informations, consultez [SETUP.md](SETUP.md)

