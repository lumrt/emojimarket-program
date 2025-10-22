use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EmojimarketInstruction 
{
	/// Create a post
	/// Accounts: payer (signer), post_pda (write), escrow_token_account (write), system_program, token_program, rent
	CreatePost {
		post_id: u64,
		start_ts: i64,
		end_ts: i64,
		creator_fee_bps: u16,
		cid: String,
	},


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