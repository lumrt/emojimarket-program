# Setup Guide - Emoji Market Program

## 🚀 Installation rapide

### 1. Installer Solana CLI

```bash
# Option A: Script automatique (recommandé)
./install-solana.sh

# Option B: Installation manuelle
sh -c "$(curl -sSfL https://release.solana.com/v1.18.22/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

### 2. Recharger le shell

```bash
# Pour bash
source ~/.bashrc

# Pour zsh
source ~/.zshrc

# Ou redémarrer le terminal
```

### 3. Vérifier l'installation

```bash
solana --version
# Devrait afficher: solana-cli 1.18.22

cargo-build-sbf --version
# Devrait afficher la version du compilateur BPF
```

### 4. Builder le programme

```bash
./build.sh
```

### 5. Lancer les tests

```bash
./test.sh
```

## 📋 Prérequis détaillés

### Rust (déjà installé ✅)

Vous avez déjà Rust installé. Vérifiez avec:
```bash
rustc --version
cargo --version
```

### Solana CLI (à installer)

Le Solana CLI fournit:
- `solana` - Client CLI pour interagir avec le réseau
- `solana-test-validator` - Validateur local pour les tests
- `cargo-build-sbf` / `cargo-build-bpf` - Compilateurs pour programmes Solana
- `solana-keygen` - Génération de paires de clés
- `spl-token` - Gestion des tokens SPL

Installation:
```bash
./install-solana.sh
```

Cela installera Solana v1.18.22 dans `~/.local/share/solana/`

### Node.js et npm (optionnel pour les scripts TypeScript)

Si vous voulez exécuter les scripts TypeScript:
```bash
# Vérifier
node --version
npm --version

# Installer les dépendances
npm install
```

## 🔧 Configuration après installation

### 1. Configurer le cluster

```bash
# Pour développement local
solana config set --url localhost

# Pour testnet
solana config set --url devnet

# Pour production
solana config set --url mainnet-beta
```

### 2. Créer un wallet de développement

```bash
# Générer une nouvelle paire de clés
solana-keygen new --outfile ~/.config/solana/id.json

# Airdrop de SOL sur devnet
solana airdrop 2

# Vérifier le solde
solana balance
```

## 🏗️ Build du programme

### Méthode 1: Script automatique

```bash
./build.sh
```

### Méthode 2: Commande directe

```bash
# Avec cargo-build-sbf (recommandé, Solana moderne)
cargo build-sbf

# Ou avec cargo-build-bpf (ancien)
cargo build-bpf
```

### Méthode 3: Makefile

```bash
make build
```

Le fichier `.so` compilé sera dans `target/deploy/emojimarket_program.so`

## 🧪 Tests

### Tests complets automatisés

```bash
./test.sh
```

Ce script:
1. Nettoie l'environnement
2. Build le programme
3. Lance un validateur local
4. Déploie le programme
5. Crée des tokens de test
6. Exécute les tests

### Tests manuels étape par étape

```bash
# 1. Démarrer le validateur local
solana-test-validator --reset --ledger ./.test-ledger &

# 2. Attendre que le validateur soit prêt
sleep 5

# 3. Configurer sur localhost
solana config set --url localhost

# 4. Builder
./build.sh

# 5. Déployer
solana program deploy target/deploy/emojimarket_program.so

# 6. Tester
npm run test

# 7. Arrêter le validateur
pkill -f solana-test-validator
```

## ❓ Dépannage

### Erreur: `cargo-build-sbf: command not found`

**Problème**: Solana CLI n'est pas installé ou pas dans le PATH

**Solution**:
```bash
# Installer
./install-solana.sh

# Ajouter au PATH pour la session actuelle
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Recharger le shell
source ~/.bashrc  # ou ~/.zshrc
```

### Erreur: `error loading target specification: could not find specification for target "bpfel-unknown-unknown"`

**Problème**: Tentative de build sans les outils Solana

**Solution**: Utilisez `cargo-build-sbf` au lieu de `cargo build --target bpfel-unknown-unknown`

### Erreur: `solana-test-validator: command not found`

**Problème**: Solana CLI non installé

**Solution**: Installez avec `./install-solana.sh`

### Le build est lent

**Normal**: La première compilation prend 1-2 minutes car elle télécharge et compile toutes les dépendances. Les builds suivants sont beaucoup plus rapides grâce au cache.

### Port 8899 déjà utilisé

**Problème**: Un validateur est déjà en cours d'exécution

**Solution**:
```bash
# Arrêter les validateurs existants
pkill -f solana-test-validator

# Attendre quelques secondes
sleep 2

# Relancer
./test.sh
```

## 🔍 Vérifier l'installation

```bash
# Vérifier toutes les commandes nécessaires
echo "=== Rust ==="
rustc --version
cargo --version

echo -e "\n=== Solana ==="
solana --version
solana-test-validator --version
cargo-build-sbf --version

echo -e "\n=== Node.js (optionnel) ==="
node --version
npm --version

echo -e "\n=== Configuration Solana ==="
solana config get
```

Si toutes ces commandes fonctionnent, vous êtes prêt ! 🎉

## 📚 Liens utiles

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana CLI Reference](https://docs.solana.com/cli)
- [BPF Developer Guide](https://docs.solana.com/developing/on-chain-programs/developing-rust)

## 💡 Prochaines étapes

Une fois l'installation terminée:

1. **Développement local**: Utilisez `./test.sh` pour tester rapidement
2. **Tests sur devnet**: Déployez sur devnet pour tester en conditions réelles
3. **Production**: Déployez sur mainnet-beta quand tout est testé

Consultez le [README.md](README.md) pour l'utilisation du programme et l'[IMPLEMENTATION.md](IMPLEMENTATION.md) pour les détails techniques.

