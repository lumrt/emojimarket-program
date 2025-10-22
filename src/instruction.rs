use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EmojimarketInstruction 
{

	/// Create a new prediction post / market
     	/// Accounts expected:
	
	/// 0. `[signer]` Creator / payer
	/// 1. `[writable]` Post account PDA (derived with seeds ["post", post_id])
	/// 2. `[writable]` Escrow token account PDA (derived with seeds ["escrow", post_id])
	/// 3. `[]` USDC mint account
	/// 4. `[]` System program
	/// 5. `[]` SPL Token program
	/// 6. `[]` Rent sysvar


	/// Create a post
	/// Accounts: payer (signer), post_pda (write), escrow_token_account (write), system_program, token_program, rent
	CreatePost {
		post_id: u64,
		start_ts: i64,
		end_ts: i64,
		creator_fee_bps: u16,
		cid: String,
	},

	/// 0. `[signer]` Bettor
	/// 1. `[writable]` Bettor's SPL token account (USDC)
	/// 2. `[writable]` Escrow SPL token account PDA (["escrow", post_id])
	/// 3. `[writable]` Post account PDA (["post", post_id])
	/// 4. `[writable]` Bet account PDA (["bet", post_id, bettor_pubkey])
	/// 5. `[]` USDC mint (same as escrow & bettor token account mint)
	/// 6. `[]` SPL Token program
	/// 7. `[]` Clock sysvar
	
	/// Place a bet
	/// Accounts: bettor (signer), bettor_token_account, escrow_token_account, post_pda , bet_pda, token_program, clock
	PlaceBet
	{
		post_id: u64,
		bet_id: u64,
		emoji_code: u32,
		quantity: u64,
		amount: u64,
		// price_per_emoji_offchain: u64
	},


	/// Resolve market (authority or creator)
	ResolveMarket
	{
		post_id: u64,
		winner_emoji: u32,
	},


	/// Claim payout (pull pattern)
	ClaimPayout 
	{
		post_id: u64,
	},
}