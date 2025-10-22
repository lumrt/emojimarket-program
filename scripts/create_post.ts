import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  getMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import * as fs from "fs";
import * as borsh from "borsh";

const PROGRAM_PATH = "target/deploy/emojimarket_program-keypair.json";
const LOCAL_USDC_PATH = "./usdc_mint.json"; // pour stocker ton mint
const LOCAL_WALLET_PATH = "~/.config/solana/id.json";
const LOCAL_RPC = "http://127.0.0.1:8899";

// ------------------ STRUCT BORSH ------------------
class CreatePost {
	post_id: bigint = BigInt(0);
	start_ts: bigint = BigInt(0);
	end_ts: bigint = BigInt(0);
	creator_fee_bps: number = 0;
	cid: string = "";
	constructor(fields: Partial<CreatePost>) {
		Object.assign(this, fields);
	}
}
const CreatePostSchema = new Map([
  [
    CreatePost,
    {
      kind: "struct",
      fields: [
        ["post_id", "u64"],
        ["start_ts", "i64"],
        ["end_ts", "i64"],
        ["creator_fee_bps", "u16"],
        ["cid", "string"],
      ],
    },
  ],
]);

// ------------------ MAIN SCRIPT ------------------
(async () => {
  const connection = new Connection(LOCAL_RPC, "confirmed");

  // Load program keypair
  const programKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync(PROGRAM_PATH, "utf8")))
  );
  const programId = programKeypair.publicKey;
  console.log("🧠 Program ID:", programId.toBase58());

  // Load wallet
  const walletKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync(
      LOCAL_WALLET_PATH.replace("~", process.env.HOME || ""), "utf8"
    )))
  );
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

  // Build instruction data (borsh encode)
  const instructionData = borsh.serialize(
    CreatePostSchema as unknown as borsh.Schema,
    new CreatePost({
      post_id: BigInt(postId),
      start_ts: BigInt(Math.floor(Date.now() / 1000)),
      end_ts: BigInt(Math.floor(Date.now() / 1000) + 3600),
      creator_fee_bps: 500, // 5%
      cid: "QmFakeCIDExampleForTest123",
    })
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
      { pubkey: PublicKey.findProgramAddressSync(
          [Buffer.from("SysvarRent111111111111111111111111111111111"),],
          SystemProgram.programId
        )[0],
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.from(Uint8Array.from(instructionData)),
  });

  // Send transaction
  const tx = new Transaction().add(ix);
  const signature = await sendAndConfirmTransaction(connection, tx, [walletKeypair]);
  console.log("✅ Transaction sent!");
  console.log("🧾 Signature:", signature);
})();
