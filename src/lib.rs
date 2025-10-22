//! emojimarket_program: minimal Solana program skeleton (no Framework)

pub mod state;
pub mod instruction;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
	program_id: &Pubkey,
	accounts: &[solana_program::account_info::AccountInfo],
	input: &[u8],
) -> ProgramResult 
{
	processor::process(program_id, accounts, input)
}