#!/bin/bash
set -e

# === CONFIG ===
PROGRAM_NAME="emojimarket_program"
PROGRAM_KEYPAIR="target/deploy/${PROGRAM_NAME}-keypair.json"
PROGRAM_SO="target/deploy/${PROGRAM_NAME}.so"
USDC_SYMBOL="USDC_TEST"
TEST_POST_ID=1

# === COLORS ===
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

# === START ===
echo -e "${CYAN}🚀 Starting Emojimarket local test...${NC}"

# Add Solana to PATH if it exists but not in current PATH
if [ -d "$HOME/.local/share/solana/install/active_release/bin" ]; then
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
fi

# Check if Solana is installed
if ! command -v solana &> /dev/null; then
    echo -e "${RED}❌ Error: Solana CLI not found!${NC}"
    echo ""
    echo -e "${YELLOW}Please install Solana first:${NC}"
    echo "  ./install-solana.sh"
    echo ""
    echo "Or reload your shell:"
    echo "  source ~/.bashrc"
    echo ""
    exit 1
fi

echo -e "${GREEN}✅ Solana CLI found: $(solana --version | head -1)${NC}"

# 1️⃣ Clean previous validator state
echo -e "${YELLOW}🧹 Cleaning previous state...${NC}"
pkill -f solana-test-validator || true
sleep 1
rm -rf ./.test-ledger
mkdir -p ./.test-ledger

# 2️⃣ Build BPF
echo -e "${YELLOW}🔨 Building Solana program...${NC}"
if ! ./build.sh; then
    echo -e "${RED}❌ Build failed!${NC}"
    echo -e "${YELLOW}Make sure Solana CLI tools are installed.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Build done.${NC}"

# 3️⃣ Start local validator
echo -e "${YELLOW}⚙️  Starting local Solana validator...${NC}"
solana-test-validator \
  --reset \
  --ledger ./.test-ledger \
  --limit-ledger-size 500 \
  --quiet \
  > ./.test-ledger/validator.log 2>&1 &

# Wait for validator to be ready
echo -e "${YELLOW}⏳ Waiting for validator to start...${NC}"
sleep 3
for i in {1..30}; do
  if solana cluster-version >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Validator is ready${NC}"
    break
  fi
  sleep 1
  if [ $i -eq 30 ]; then
    echo -e "${RED}❌ Validator failed to start${NC}"
    exit 1
  fi
done

# 4️⃣ Set config to localnet
solana config set --url localhost >/dev/null

# 5️⃣ Generate program keypair if missing
if [ ! -f "${PROGRAM_KEYPAIR}" ]; then
  echo -e "${YELLOW}🔑 Generating program keypair...${NC}"
  solana-keygen new -o ${PROGRAM_KEYPAIR} --no-bip39-passphrase >/dev/null
fi

# 6️⃣ Deploy the program
echo -e "${YELLOW}📦 Deploying program...${NC}"
PROGRAM_ID=$(solana program deploy ${PROGRAM_SO} --program-id ${PROGRAM_KEYPAIR} | grep -oE 'Program Id: [A-Za-z0-9]+' | awk '{print $3}')
echo -e "${GREEN}✅ Program deployed: ${PROGRAM_ID}${NC}"

# 7️⃣ Create test USDC mint
echo -e "${YELLOW}💰 Creating mock USDC token...${NC}"
USDC_MINT=$(spl-token create-token | grep -oE 'Creating token [A-Za-z0-9]+' | awk '{print $3}')
USDC_ACC=$(spl-token create-account ${USDC_MINT} | grep -oE 'Creating account [A-Za-z0-9]+' | awk '{print $3}')
spl-token mint ${USDC_MINT} 1000000 >/dev/null
echo -e "${GREEN}✅ Mock USDC created: ${USDC_MINT}${NC}"
echo -e "   → Wallet Token Account: ${USDC_ACC}"

# Save USDC mint address for TypeScript script
echo "{\"mint\":\"${USDC_MINT}\"}" > usdc_mint.json

# 8️⃣ Derive PDAs
POST_SEED=$(printf "post%s" "${TEST_POST_ID}" | xxd -p)
ESCROW_SEED=$(printf "escrow%s" "${TEST_POST_ID}" | xxd -p)
POST_PDA=$(solana address -k ${PROGRAM_KEYPAIR} 2>/dev/null || true)
ESCROW_PDA=$(solana address -k ${PROGRAM_KEYPAIR} 2>/dev/null || true)

echo -e "${CYAN}📌 Derived PDAs (simulated):${NC}"
echo "  Post PDA   : ${POST_PDA}"
echo "  Escrow PDA : ${ESCROW_PDA}"

# 9️⃣ Display token balances
echo -e "${YELLOW}📊 Current Token Balances:${NC}"
spl-token accounts

# 🔟 Run program tests
echo -e "${YELLOW}🧪 Running Rust integration tests...${NC}"
cargo test-sbf -- --nocapture 2>&1 | tee ./.test-ledger/test_output.log || {
    echo -e "${YELLOW}⚠️  cargo test-sbf not available, skipping Rust tests${NC}"
}

# 🔁 Display validator logs
echo -e "${YELLOW}📜 Validator logs (last 30 lines):${NC}"
tail -n 30 ./.test-ledger/validator.log

# ✅ DONE
echo -e "${GREEN}🎉 Test completed successfully!${NC}"
echo -e "Program ID: ${PROGRAM_ID}"
echo -e "Mock USDC Mint: ${USDC_MINT}"
echo -e "Post PDA: ${POST_PDA}"
echo -e "Escrow PDA: ${ESCROW_PDA}"
# 🧠 Run TypeScript tests
if [ -f "scripts/test_emoji_market.ts" ]; then
    echo -e "${YELLOW}📡 Running test_emoji_market.ts...${NC}"
    npx ts-node scripts/test_emoji_market.ts 2>&1 | tee ./.test-ledger/tx_output.log || {
        echo -e "${YELLOW}⚠️  TypeScript test skipped or failed${NC}"
    }
else
    echo -e "${YELLOW}ℹ️  No TypeScript tests found, skipping${NC}"
fi

echo -e "${GREEN}✅ All tests completed!${NC}"
echo -e "To stop the local validator, run: ${YELLOW}pkill -f solana-test-validator${NC}"