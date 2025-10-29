/**
 * Example Usage Script for EmojiMarket Program
 * Demonstrates basic queries and PDA calculations
 */

import { EmojiMarketClient, PDAs } from "./client";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import * as fs from "fs";

// ============================================================================
// Configuration
// ============================================================================

const CLUSTER: "devnet" | "localhost" = "devnet";
const PROGRAM_ID = new PublicKey("ZTnFhxro4BiVWvAhH6m11SJx4BUDieP2Vu4yYymco1u");

// Popular emojis for testing
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

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Load wallet from filesystem or create new one
 */
function loadOrCreateWallet(walletPath?: string): Keypair {
  if (walletPath && fs.existsSync(walletPath)) {
    console.log(`📂 Loading wallet from ${walletPath}`);
    const secretKey = JSON.parse(fs.readFileSync(walletPath, "utf-8"));
    return Keypair.fromSecretKey(Uint8Array.from(secretKey));
  }

  console.log("🆕 Creating new wallet for demo...");
  const wallet = Keypair.generate();
  console.log(`   Public Key: ${wallet.publicKey.toBase58()}`);
  return wallet;
}

/**
 * Ensure wallet has enough balance
 */
async function ensureBalance(
  client: EmojiMarketClient,
  publicKey: PublicKey,
  minBalance: number = 1
): Promise<void> {
  const balance = await client.getBalance(publicKey);
  console.log(`💰 Current balance: ${balance.toFixed(4)} SOL`);

  if (balance < minBalance && CLUSTER === "devnet") {
    console.log(`⚠️  Balance too low, requesting airdrop...`);
    await client.airdrop(publicKey, 2);
    const newBalance = await client.getBalance(publicKey);
    console.log(`✅ New balance: ${newBalance.toFixed(4)} SOL`);
  }
}

// ============================================================================
// Example Scenarios
// ============================================================================

/**
 * Scenario 1: Display Program Information
 */
async function scenario1_ProgramInfo(client: EmojiMarketClient): Promise<void> {
  console.log("\n╔════════════════════════════════════════════╗");
  console.log("║   SCENARIO 1: Program Information         ║");
  console.log("╚════════════════════════════════════════════╝\n");

  console.log(`📝 Program ID: ${client.programId.toBase58()}`);
  console.log(`🌐 Cluster: ${CLUSTER}`);
  console.log(`🔗 Explorer: https://explorer.solana.com/address/${client.programId.toBase58()}?cluster=${CLUSTER}\n`);

  // Check if program exists
  const programExists = await client.accountExists(client.programId);
  console.log(`Program exists: ${programExists ? "✅ Yes" : "❌ No"}`);

  if (programExists) {
    const programInfo = await client.getAccountInfo(client.programId);
    if (programInfo) {
      console.log(`Program is executable: ${programInfo.executable ? "✅ Yes" : "❌ No"}`);
      console.log(`Program data length: ${programInfo.data.length} bytes`);
      console.log(`Program owner: ${programInfo.owner.toBase58()}`);
    }
  }
}

/**
 * Scenario 2: Display PDAs
 */
async function scenario2_DisplayPDAs(client: EmojiMarketClient): Promise<void> {
  console.log("\n╔════════════════════════════════════════════╗");
  console.log("║        SCENARIO 2: Display PDAs           ║");
  console.log("╚════════════════════════════════════════════╝\n");

  // Config PDA
  const [configPDA, configBump] = PDAs.getConfigPDA(client.programId);
  console.log("🔧 Config PDA:");
  console.log(`   Address: ${configPDA.toBase58()}`);
  console.log(`   Bump: ${configBump}`);
  
  const configExists = await client.accountExists(configPDA);
  console.log(`   Exists: ${configExists ? "✅ Yes" : "❌ No"}\n`);

  // Example Market PDA
  const exampleMarketId = new BN(12345);
  const [marketPDA, marketBump] = PDAs.getMarketPDA(exampleMarketId, client.programId);
  console.log(`📊 Example Market PDA (ID: ${exampleMarketId.toString()}):`);
  console.log(`   Address: ${marketPDA.toBase58()}`);
  console.log(`   Bump: ${marketBump}`);
  
  const marketExists = await client.accountExists(marketPDA);
  console.log(`   Exists: ${marketExists ? "✅ Yes" : "❌ No"}\n`);

  // Vault PDA
  const [vaultPDA, vaultBump] = PDAs.getMarketVaultPDA(marketPDA, client.programId);
  console.log("💰 Market Vault PDA:");
  console.log(`   Address: ${vaultPDA.toBase58()}`);
  console.log(`   Bump: ${vaultBump}\n`);

  // Bet Account PDA
  const userPubkey = client.provider.wallet.publicKey;
  const [betPDA, betBump] = PDAs.getBetAccountPDA(marketPDA, userPubkey, client.programId);
  console.log("🎫 Example Bet Account PDA:");
  console.log(`   User: ${userPubkey.toBase58()}`);
  console.log(`   Address: ${betPDA.toBase58()}`);
  console.log(`   Bump: ${betBump}\n`);
}

/**
 * Scenario 3: Query All Program Accounts
 */
async function scenario3_QueryAccounts(client: EmojiMarketClient): Promise<void> {
  console.log("\n╔════════════════════════════════════════════╗");
  console.log("║    SCENARIO 3: Query Program Accounts    ║");
  console.log("╚════════════════════════════════════════════╝\n");

  console.log("🔍 Fetching all program accounts...");
  const accounts = await client.getProgramAccounts();
  console.log(`\n📊 Found ${accounts.length} accounts:\n`);

  if (accounts.length === 0) {
    console.log("   No accounts found. The program might not be initialized yet.");
    console.log("   💡 Initialize the config first using the admin wallet.\n");
    return;
  }

  accounts.forEach((account, index) => {
    console.log(`--- Account ${index + 1} ---`);
    console.log(`Address: ${account.pubkey.toBase58()}`);
    console.log(`Data Length: ${account.account.data.length} bytes`);
    console.log(`Owner: ${account.account.owner.toBase58()}`);
    console.log(`Lamports: ${account.account.lamports / LAMPORTS_PER_SOL} SOL\n`);
  });
}

/**
 * Scenario 4: Display Emoji Information
 */
async function scenario4_EmojiInfo(): Promise<void> {
  console.log("\n╔════════════════════════════════════════════╗");
  console.log("║      SCENARIO 4: Emoji Information        ║");
  console.log("╚════════════════════════════════════════════╝\n");

  console.log("🎨 Available Emojis for Betting:\n");
  
  Object.entries(EMOJIS).forEach(([name, id]) => {
    const emoji = String.fromCodePoint(id);
    console.log(`${emoji}  ${name.padEnd(12)} - ID: ${id} (0x${id.toString(16).toUpperCase()})`);
  });

  console.log("\n💡 Tips:");
  console.log("   - Use the emoji ID (decimal or hex) when placing bets");
  console.log("   - You can add more emojis by finding their Unicode codepoint");
  console.log("   - Visit https://unicode.org/emoji/charts/emoji-list.html\n");
}

/**
 * Scenario 5: Calculate Market Example
 */
async function scenario5_MarketExample(): Promise<void> {
  console.log("\n╔════════════════════════════════════════════╗");
  console.log("║     SCENARIO 5: Market Calculation        ║");
  console.log("╚════════════════════════════════════════════╝\n");

  const now = Math.floor(Date.now() / 1000);
  const oneHour = 3600;
  const oneDay = 86400;
  const oneWeek = 604800;

  console.log("⏰ Market Duration Examples:\n");
  console.log(`Current Unix Timestamp: ${now}`);
  console.log(`1 Hour from now: ${now + oneHour}`);
  console.log(`1 Day from now: ${now + oneDay}`);
  console.log(`1 Week from now: ${now + oneWeek}\n`);

  console.log("📊 Market ID Examples:\n");
  console.log(`Using timestamp: new BN(${now})`);
  console.log(`Using sequential: new BN(1), new BN(2), new BN(3)...`);
  console.log(`Using random: new BN(Math.floor(Math.random() * 1000000))\n`);

  console.log("💰 Pricing Examples (with default config):\n");
  console.log("Base Price: 0.001 SOL per vote");
  console.log("Platform Fee: 2.5%");
  console.log("Creator Fee: 2.5%");
  console.log("Total Fees: 5%\n");

  console.log("Example bet costs:");
  console.log("  5 votes  ≈ 0.005 SOL");
  console.log(" 10 votes  ≈ 0.010 SOL");
  console.log(" 50 votes  ≈ 0.050 SOL");
  console.log("100 votes  ≈ 0.100 SOL\n");
}

/**
 * Scenario 6: Check Wallet Status
 */
async function scenario6_WalletStatus(client: EmojiMarketClient): Promise<void> {
  console.log("\n╔════════════════════════════════════════════╗");
  console.log("║       SCENARIO 6: Wallet Status           ║");
  console.log("╚════════════════════════════════════════════╝\n");

  const wallet = client.provider.wallet.publicKey;
  console.log(`👤 Wallet Address: ${wallet.toBase58()}`);
  console.log(`🔗 Explorer: https://explorer.solana.com/address/${wallet.toBase58()}?cluster=${CLUSTER}\n`);

  const balance = await client.getBalance(wallet);
  console.log(`💰 Balance: ${balance.toFixed(4)} SOL\n`);

  if (balance < 0.1 && CLUSTER === "devnet") {
    console.log("⚠️  Low balance detected!");
    console.log("💡 Get devnet SOL from:");
    console.log("   - CLI: solana airdrop 2");
    console.log("   - Web: https://faucet.solana.com\n");
  }
}

// ============================================================================
// Main Execution
// ============================================================================

async function main() {
  console.log("╔════════════════════════════════════════════════════╗");
  console.log("║     EmojiMarket Program - Example Usage Script    ║");
  console.log("╚════════════════════════════════════════════════════╝\n");

  // Load wallet
  const walletPath = process.env.WALLET_PATH;
  const keypair = loadOrCreateWallet(walletPath);

  // Setup client
  const connection = EmojiMarketClient.createConnection(CLUSTER);
  const wallet = new anchor.Wallet(keypair);
  const provider = EmojiMarketClient.createProvider(connection, wallet);
  const client = new EmojiMarketClient(provider, PROGRAM_ID);

  console.log(`🌐 Cluster: ${CLUSTER}`);
  console.log(`👤 Wallet: ${keypair.publicKey.toBase58()}\n`);

  try {
    // Run all scenarios
    await scenario1_ProgramInfo(client);
    await scenario2_DisplayPDAs(client);
    await scenario3_QueryAccounts(client);
    await scenario4_EmojiInfo();
    await scenario5_MarketExample();
    await scenario6_WalletStatus(client);

    console.log("\n" + "=".repeat(52));
    console.log("✨ All scenarios completed!");
    console.log("=".repeat(52));
    console.log("\n💡 Next Steps:");
    console.log("   1. Initialize the config (admin only)");
    console.log("   2. Create markets");
    console.log("   3. Place bets");
    console.log("   4. End markets and claim rewards");
    console.log("\n📚 For full implementation, you'll need to:");
    console.log("   - Generate IDL with 'anchor build'");
    console.log("   - Or implement manual instruction building with borsh");
    console.log("");
  } catch (error) {
    console.error("\n💥 Error:", error);
    process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  main().catch((error) => {
    console.error("Fatal error:", error);
    process.exit(1);
  });
}
