import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  getMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import * as fs from "fs";
import { Buffer } from "buffer";

const PROGRAM_PATH = "target/deploy/emojimarket_program-keypair.json";
const LOCAL_USDC_PATH = "./usdc_mint.json";
const LOCAL_RPC = "http://127.0.0.1:8899";

// Helper to encode instruction data manually
function encodeCreatePostInstruction(
  postId: bigint,
  startTs: bigint,
  endTs: bigint,
  creatorFeeBps: number,
  cid: string
): Buffer {
  // Instruction discriminator (0 for CreatePost)
  const discriminator = Buffer.alloc(1);
  discriminator.writeUInt8(0, 0);
  
  // post_id (u64)
  const postIdBuf = Buffer.alloc(8);
  postIdBuf.writeBigUInt64LE(postId, 0);
  
  // start_ts (i64)
  const startTsBuf = Buffer.alloc(8);
  startTsBuf.writeBigInt64LE(startTs, 0);
  
  // end_ts (i64)
  const endTsBuf = Buffer.alloc(8);
  endTsBuf.writeBigInt64LE(endTs, 0);
  
  // creator_fee_bps (u16)
  const feeBuf = Buffer.alloc(2);
  feeBuf.writeUInt16LE(creatorFeeBps, 0);
  
  // cid (string) - borsh string: u32 length + utf8 bytes
  const cidBytes = Buffer.from(cid, "utf8");
  const cidLenBuf = Buffer.alloc(4);
  cidLenBuf.writeUInt32LE(cidBytes.length, 0);
  
  return Buffer.concat([
    discriminator,
    postIdBuf,
    startTsBuf,
    endTsBuf,
    feeBuf,
    cidLenBuf,
    cidBytes
  ]);
}

// ------------------ MAIN SCRIPT ------------------
(async () => {
  const connection = new Connection(LOCAL_RPC, "confirmed");

  // Load program keypair
  const programKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync(PROGRAM_PATH, "utf8")))
  );
  const programId = programKeypair.publicKey;
  console.log("🧠 Program ID:", programId.toBase58());

  // Load wallet - try default location or create new keypair for testing
  let walletKeypair: Keypair;
  const walletPath = `${process.env.HOME}/.config/solana/id.json`;
  try {
    walletKeypair = Keypair.fromSecretKey(
      Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, "utf8")))
    );
  } catch (e) {
    console.log("⚠️  No wallet found, using program keypair as payer for demo");
    walletKeypair = programKeypair;
  }
  console.log("👤 Wallet:", walletKeypair.publicKey.toBase58());

  // Load or create mock USDC mint
  const usdcMintAddress = JSON.parse(fs.readFileSync(LOCAL_USDC_PATH, "utf8")).mint;
  const usdcMint = new PublicKey(usdcMintAddress);
  const mintInfo = await getMint(connection, usdcMint);
  console.log("💰 USDC Mint:", mintInfo.address.toBase58());

  // Derive PDAs
  const postId = 1;
  const [postPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("post"), Buffer.from(new Uint8Array(new BigUint64Array([BigInt(postId)]).buffer))],
    programId
  );
  const [escrowPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), Buffer.from(new Uint8Array(new BigUint64Array([BigInt(postId)]).buffer))],
    programId
  );

  console.log("📦 Post PDA  :", postPda.toBase58());
  console.log("💼 Escrow PDA:", escrowPda.toBase58());

  // Build instruction data
  const instructionData = encodeCreatePostInstruction(
    BigInt(postId),
    BigInt(Math.floor(Date.now() / 1000)),
    BigInt(Math.floor(Date.now() / 1000) + 3600),
    500, // 5%
    "QmFakeCIDExampleForTest123"
  );

  // Build instruction
  const ix = new TransactionInstruction({
    programId,
    keys: [
      { pubkey: walletKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: postPda, isSigner: false, isWritable: true },
      { pubkey: escrowPda, isSigner: false, isWritable: true },
      { pubkey: usdcMint, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    data: instructionData,
  });

  // Send transaction
  const tx = new Transaction().add(ix);
  const signature = await sendAndConfirmTransaction(connection, tx, [walletKeypair]);
  console.log("✅ Transaction sent!");
  console.log("🧾 Signature:", signature);
})();
