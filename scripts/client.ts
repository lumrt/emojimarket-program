/**
 * EmojiMarket Program Client
 * Client TypeScript complet pour interagir avec le programme EmojiMarket
 */

import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, BN } from "@coral-xyz/anchor";
import { 
  PublicKey, 
  Keypair, 
  SystemProgram,
  LAMPORTS_PER_SOL,
  Connection,
  clusterApiUrl,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";

// ============================================================================
// Configuration & Types
// ============================================================================

const PROGRAM_ID = new PublicKey("ZTnFhxro4BiVWvAhH6m11SJx4BUDieP2Vu4yYymco1u");

interface ConfigParams {
  adminAddress: PublicKey;
  platformFeeBps: number;
  creatorFeeBps: number;
  basePriceLamports: BN;
  malusKMillis: number;
  quadAMicros: BN;
  quadBMicros: BN;
  minDurationSecs: number;
  maxDurationSecs: number;
}

interface MarketData {
  creator: PublicKey;
  title: string;
  imageUrl: string | null;
  startTs: BN;
  endTs: BN;
  status: number;
  totalPot: BN;
  totalVotes: BN;
  emojiIds: number[];
  emojiVotes: BN[];
  winner: number | null;
  platformFeeTaken: BN;
  creatorFeeTaken: BN;
}

interface BetData {
  market: PublicKey;
  user: PublicKey;
  emojiIds: number[];
  emojiVotes: BN[];
  totalSpent: BN;
  claimed: boolean;
}

// ============================================================================
// PDA Helpers
// ============================================================================

class PDAHelper {
  /**
   * Dérive l'adresse PDA du compte Config
   */
  static getConfigPDA(programId: PublicKey = PROGRAM_ID): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      programId
    );
  }

  /**
   * Dérive l'adresse PDA d'un Market
   */
  static getMarketPDA(
    marketId: BN,
    programId: PublicKey = PROGRAM_ID
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("market"), marketId.toArrayLike(Buffer, "le", 8)],
      programId
    );
  }

  /**
   * Dérive l'adresse PDA d'un BetAccount
   */
  static getBetAccountPDA(
    market: PublicKey,
    user: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("bet"), market.toBuffer(), user.toBuffer()],
      programId
    );
  }

  /**
   * Dérive l'adresse PDA du vault du market
   */
  static getMarketVaultPDA(
    market: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("market_vault"), market.toBuffer()],
      programId
    );
  }
}

// Export PDAHelper as PDAs for backwards compatibility
export const PDAs = PDAHelper;

// ============================================================================
// Client Class
// ============================================================================

export class EmojiMarketClient {
  provider: AnchorProvider;
  connection: Connection;
  programId: PublicKey;

  constructor(
    provider: AnchorProvider,
    programId: PublicKey = PROGRAM_ID
  ) {
    this.provider = provider;
    this.connection = provider.connection;
    this.programId = programId;
  }

  /**
   * Crée une connexion au cluster spécifié
   */
  static createConnection(cluster: "devnet" | "testnet" | "mainnet-beta" | "localhost" = "devnet"): Connection {
    if (cluster === "localhost") {
      return new Connection("http://localhost:8899", "confirmed");
    }
    return new Connection(clusterApiUrl(cluster), "confirmed");
  }

  /**
   * Initialise le provider avec un wallet
   */
  static createProvider(
    connection: Connection,
    wallet: anchor.Wallet
  ): AnchorProvider {
    return new AnchorProvider(connection, wallet, {
      commitment: "confirmed",
      preflightCommitment: "confirmed",
    });
  }

  /**
   * Charge l'IDL depuis le filesystem ou retourne null
   */
  private loadIDL(): any | null {
    const idlPath = path.join(__dirname, "../target/idl/emojimarket_program.json");
    if (fs.existsSync(idlPath)) {
      return JSON.parse(fs.readFileSync(idlPath, "utf-8"));
    }
    return null;
  }

  // ============================================================================
  // Instructions (à implémenter avec des instructions raw si nécessaire)
  // ============================================================================

  /**
   * Note: Ces méthodes nécessitent l'IDL généré par Anchor.
   * Pour l'instant, elles sont des placeholders.
   * Pour une implémentation complète sans IDL, il faudrait construire
   * les instructions manuellement avec borsh serialization.
   */

  async initializeConfig(params: ConfigParams): Promise<string> {
    throw new Error(
      "Cette méthode nécessite l'IDL d'Anchor. " +
      "Exécutez 'anchor build' et assurez-vous que l'IDL est généré dans target/idl/"
    );
  }

  async createMarket(
    marketId: BN,
    title: string,
    imageUrl: string | null,
    durationSeconds: number
  ): Promise<string> {
    throw new Error(
      "Cette méthode nécessite l'IDL d'Anchor. " +
      "Exécutez 'anchor build' et assurez-vous que l'IDL est généré dans target/idl/"
    );
  }

  async placeBet(
    marketId: BN,
    emojiId: number,
    voteQty: BN
  ): Promise<string> {
    throw new Error(
      "Cette méthode nécessite l'IDL d'Anchor. " +
      "Exécutez 'anchor build' et assurez-vous que l'IDL est généré dans target/idl/"
    );
  }

  async endMarket(marketId: BN): Promise<string> {
    throw new Error(
      "Cette méthode nécessite l'IDL d'Anchor. " +
      "Exécutez 'anchor build' et assurez-vous que l'IDL est généré dans target/idl/"
    );
  }

  async claim(marketId: BN, user?: PublicKey): Promise<string> {
    throw new Error(
      "Cette méthode nécessite l'IDL d'Anchor. " +
      "Exécutez 'anchor build' et assurez-vous que l'IDL est généré dans target/idl/"
    );
  }

  // ============================================================================
  // Queries / Getters
  // ============================================================================

  /**
   * Récupère les données d'un compte
   */
  async getAccountInfo(address: PublicKey): Promise<any> {
    try {
      const accountInfo = await this.connection.getAccountInfo(address);
      return accountInfo;
    } catch (error) {
      console.error("Error fetching account:", error);
      return null;
    }
  }

  /**
   * Récupère les données de configuration
   */
  async getConfig(): Promise<any> {
    const [configPDA] = PDAHelper.getConfigPDA(this.programId);
    return await this.getAccountInfo(configPDA);
  }

  /**
   * Récupère les données d'un market
   */
  async getMarket(marketId: BN): Promise<any> {
    const [marketPDA] = PDAHelper.getMarketPDA(marketId, this.programId);
    return await this.getAccountInfo(marketPDA);
  }

  /**
   * Récupère les données d'un BetAccount
   */
  async getBetAccount(
    marketId: BN,
    user?: PublicKey
  ): Promise<any> {
    const userPubkey = user || this.provider.wallet.publicKey;
    const [marketPDA] = PDAHelper.getMarketPDA(marketId, this.programId);
    const [betAccountPDA] = PDAHelper.getBetAccountPDA(
      marketPDA,
      userPubkey,
      this.programId
    );
    return await this.getAccountInfo(betAccountPDA);
  }

  /**
   * Récupère tous les accounts du programme
   */
  async getProgramAccounts(): Promise<any[]> {
    try {
      const accounts = await this.connection.getProgramAccounts(this.programId);
      return [...accounts];
    } catch (error) {
      console.error("Error fetching program accounts:", error);
      return [];
    }
  }

  /**
   * Récupère tous les markets
   */
  async getAllMarkets(): Promise<any[]> {
    try {
      const accounts = await this.getProgramAccounts();
      // Filter markets (size-based heuristic)
      // Markets are larger accounts (> 500 bytes typically)
      return accounts.filter((acc) => acc.account.data.length > 500);
    } catch (error) {
      console.error("Error fetching all markets:", error);
      return [];
    }
  }

  /**
   * Récupère tous les paris d'un utilisateur
   */
  async getUserBets(user?: PublicKey): Promise<any[]> {
    try {
      const accounts = await this.getProgramAccounts();
      // Filter bet accounts (smaller accounts, ~300-400 bytes)
      return accounts.filter((acc) => 
        acc.account.data.length > 200 && 
        acc.account.data.length < 500
      );
    } catch (error) {
      console.error("Error fetching user bets:", error);
      return [];
    }
  }

  // ============================================================================
  // Helpers
  // ============================================================================

  /**
   * Affiche les informations d'un market de manière formatée
   */
  displayMarket(market: any): void {
    console.log("\n╔════════════════════════════════════════════╗");
    console.log("║           MARKET INFORMATION              ║");
    console.log("╚════════════════════════════════════════════╝");
    
    if (!market || !market.data) {
      console.log("No market data available");
      return;
    }

    console.log(`Account: ${market.owner?.toBase58() || "N/A"}`);
    console.log(`Data Length: ${market.data?.length || 0} bytes`);
    console.log("");
  }

  /**
   * Airdrop SOL sur un compte (devnet/testnet uniquement)
   */
  async airdrop(publicKey: PublicKey, amount: number = 2): Promise<void> {
    console.log(`💰 Requesting airdrop of ${amount} SOL...`);
    try {
      const signature = await this.connection.requestAirdrop(
        publicKey,
        amount * LAMPORTS_PER_SOL
      );
      await this.connection.confirmTransaction(signature);
      console.log(`✅ Airdrop successful: ${signature}`);
    } catch (error) {
      console.error("❌ Airdrop failed:", error);
      console.log("💡 Tip: Try requesting from https://faucet.solana.com");
    }
  }

  /**
   * Récupère le solde d'un compte
   */
  async getBalance(publicKey: PublicKey): Promise<number> {
    const balance = await this.connection.getBalance(publicKey);
    return balance / LAMPORTS_PER_SOL;
  }

  /**
   * Vérifie si un compte existe
   */
  async accountExists(publicKey: PublicKey): Promise<boolean> {
    const account = await this.connection.getAccountInfo(publicKey);
    return account !== null;
  }

  /**
   * Affiche les PDAs du programme
   */
  displayPDAs(marketId?: BN): void {
    console.log("\n╔════════════════════════════════════════════╗");
    console.log("║           PROGRAM DERIVED ADDRESSES       ║");
    console.log("╚════════════════════════════════════════════╝\n");

    const [configPDA, configBump] = PDAHelper.getConfigPDA(this.programId);
    console.log("Config PDA:");
    console.log(`  Address: ${configPDA.toBase58()}`);
    console.log(`  Bump: ${configBump}\n`);

    if (marketId) {
      const [marketPDA, marketBump] = PDAHelper.getMarketPDA(marketId, this.programId);
      console.log(`Market PDA (ID: ${marketId.toString()}):`);
      console.log(`  Address: ${marketPDA.toBase58()}`);
      console.log(`  Bump: ${marketBump}\n`);

      const [vaultPDA, vaultBump] = PDAHelper.getMarketVaultPDA(marketPDA, this.programId);
      console.log("Market Vault PDA:");
      console.log(`  Address: ${vaultPDA.toBase58()}`);
      console.log(`  Bump: ${vaultBump}\n`);

      const userPubkey = this.provider.wallet.publicKey;
      const [betPDA, betBump] = PDAHelper.getBetAccountPDA(marketPDA, userPubkey, this.programId);
      console.log("Bet Account PDA (your wallet):");
      console.log(`  Address: ${betPDA.toBase58()}`);
      console.log(`  Bump: ${betBump}\n`);
    }
  }
}

// ============================================================================
// Exemple d'utilisation
// ============================================================================

async function main() {
  console.log("🎯 EmojiMarket Client - Example Usage\n");

  // Configuration
  const connection = EmojiMarketClient.createConnection("devnet");
  
  // Créer ou charger un wallet
  const wallet = new anchor.Wallet(Keypair.generate());
  
  const provider = EmojiMarketClient.createProvider(connection, wallet);
  const client = new EmojiMarketClient(provider, PROGRAM_ID);

  console.log(`🌐 Cluster: devnet`);
  console.log(`👤 Wallet: ${wallet.publicKey.toBase58()}`);
  console.log(`📝 Program ID: ${PROGRAM_ID.toBase58()}\n`);

  // Afficher les PDAs
  client.displayPDAs();

  // Exemple avec un market ID
  const exampleMarketId = new BN(12345);
  client.displayPDAs(exampleMarketId);

  // Vérifier le programme
  console.log("🔍 Checking program...");
  const programExists = await client.accountExists(PROGRAM_ID);
  console.log(`Program exists: ${programExists ? "✅ Yes" : "❌ No"}\n`);

  // Vérifier la config
  console.log("📋 Checking config...");
  const [configPDA] = PDAHelper.getConfigPDA(PROGRAM_ID);
  const configExists = await client.accountExists(configPDA);
  console.log(`Config exists: ${configExists ? "✅ Yes" : "❌ No"}\n`);

  // Récupérer tous les accounts
  console.log("📊 Fetching all program accounts...");
  const accounts = await client.getProgramAccounts();
  console.log(`Found ${accounts.length} accounts\n`);

  console.log("✨ Example completed!");
  console.log("\n💡 Note: Pour utiliser les instructions (createMarket, placeBet, etc.),");
  console.log("   vous devez générer l'IDL avec 'anchor build' ou utiliser");
  console.log("   les instructions manuelles avec borsh serialization.");
}

// Exécuter si appelé directement
if (require.main === module) {
  main().catch((error) => {
    console.error("Error:", error);
    process.exit(1);
  });
}

// Exports
export { ConfigParams, MarketData, BetData };
