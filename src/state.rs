use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;


pub const MAX_CID_BYTES: usize = 64;


#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]

pub struct Post 
{ // X -> memory size on chain
	pub is_initialized: bool, // 1
	pub post_id: u64, // 8
	pub creator: Pubkey, // 32
	pub start_ts: i64, // 8
	pub end_ts: i64, // 8
	pub pot_amount: u64, // 8 (in USDC smallest units)
	pub num_bets: u32, // 4
	pub creator_fee_bps: u16, // 2
	pub creator_fees_withdrawable: u64, // 8
	pub status: u8, // 1 (0=open,1=closed,2=resolved)
	pub cid_len: u8, // 1
	pub cid: [u8; MAX_CID_BYTES], // var
}


impl Post {
	pub fn space() -> usize 
	{
		// estimated space allocation
		1 + 8 + 32 + 8 + 8 + 8 + 4 + 2 + 8 + 1 + 1 + MAX_CID_BYTES
	}
}


#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]

pub struct Bet
{ // X -> memory size on chain
	pub is_initialized: bool, // 1
	pub bet_id: u64, // 8
	pub post_id: u64, // 8
	pub user: Pubkey, // 32
	pub emoji_code: u32, // 4
	pub quantity: u64, // 8
	pub amount: u64, // 8
	pub timestamp: i64, // 8
}


impl Bet
{
	pub fn space() -> usize 
	{
		1 + 8 + 8 + 32 + 4 + 8 + 8 + 8
	}
}