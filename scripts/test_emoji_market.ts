import { Connection, Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import * as fs from 'fs';

// Program ID
const PROGRAM_ID = new PublicKey('4Rp6sVke1a1PRxhQhZJvFbgArcy3AokWQYLrKsBvrcmR');

async function main() {
    console.log('🚀 Testing Emoji Market Program\n');

    // Connect to localnet
    const connection = new Connection('http://localhost:8899', 'confirmed');

    // Load wallet
    const walletPath = process.env.HOME + '/.config/solana/id.json';
    const walletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, 'utf-8')))
    );

    console.log('📍 Configuration:');
    console.log(`   Program ID: ${PROGRAM_ID.toBase58()}`);
    console.log(`   Wallet: ${walletKeypair.publicKey.toBase58()}`);

    // Get balance
    const balance = await connection.getBalance(walletKeypair.publicKey);
    console.log(`   Balance: ${balance / LAMPORTS_PER_SOL} SOL\n`);

    // Derive PDAs
    console.log('🔑 Deriving PDAs:');

    // Config PDA
    const [configPda, configBump] = PublicKey.findProgramAddressSync(
        [Buffer.from('config')],
        PROGRAM_ID
    );
    console.log(`   ✅ Config PDA: ${configPda.toBase58()}`);

    // Market PDA (example with market_id = 1)
    const marketId = 1;
    const [marketPda, marketBump] = PublicKey.findProgramAddressSync(
        [
            Buffer.from('market'),
            walletKeypair.publicKey.toBuffer(),
            Buffer.from(new Uint8Array(new BigUint64Array([BigInt(marketId)]).buffer))
        ],
        PROGRAM_ID
    );
    console.log(`   ✅ Market PDA: ${marketPda.toBase58()}`);

    // Bet PDA
    const [betPda, betBump] = PublicKey.findProgramAddressSync(
        [
            Buffer.from('bet'),
            marketPda.toBuffer(),
            walletKeypair.publicKey.toBuffer()
        ],
        PROGRAM_ID
    );
    console.log(`   ✅ Bet PDA: ${betPda.toBase58()}`);

    console.log('\n📊 Next steps to test the program:');
    console.log('   1. Call initialize_config to set up the platform');
    console.log('   2. Call create_market to create a prediction market');
    console.log('   3. Call bet to place votes on emojis');
    console.log('   4. Call end_market when the market closes');
    console.log('   5. Call claim for winners to get their rewards');

    console.log('\n💡 To implement full tests:');
    console.log('   - Use @coral-xyz/anchor package');
    console.log('   - Generate IDL with: anchor build');
    console.log('   - Use Anchor\'s testing framework');

    console.log('\n✅ PDA derivation test completed successfully!');
}

main().catch(err => {
    console.error('\n❌ Error:', err.message);
    process.exit(1);
});
