//! emojimarket_program: minimal Solana program skeleton (no Framework)

pub mod state;
pub mod instruction;
pub mod processor;


use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;


entrypoint!(process_instruction);


fn process_instruction(
	program_id: &Pubkey,
	accounts: &[solana_program::account_info::AccountInfo],
	input: &[u8],
) -> ProgramResult 
{
	processor::process(program_id, accounts, input)
}