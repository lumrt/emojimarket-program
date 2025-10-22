use crate::instruction::EmojimarketInstruction;
use crate::state::{Post, Bet};
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar, rent::Rent},
    program::{invoke_signed, invoke},
    system_instruction,
    system_program,
};
use spl_token::instruction as token_instruction;
use spl_token::state::Account as SplAccount;
use spl_token::state::Account;
use std::convert::TryInto;

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let mut data: &[u8] = input;
    let ix = EmojimarketInstruction::try_from_slice(&mut data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match ix {
        EmojimarketInstruction::CreatePost { post_id, start_ts, end_ts, creator_fee_bps, cid } => {
            msg!("CreatePost {}", post_id);
            process_create_post(program_id, accounts, post_id, start_ts, end_ts, creator_fee_bps, cid)
        }
        EmojimarketInstruction::PlaceBet { post_id, bet_id, emoji_code, quantity, amount } => {
            msg!("PlaceBet {} for post {}", bet_id, post_id);
            process_place_bet(program_id, accounts, post_id, bet_id, emoji_code, quantity, amount)
        }
        EmojimarketInstruction::ResolveMarket { post_id, winning_emoji } => {
            msg!("ResolveMarket {} -> {}", post_id, winning_emoji);
            process_resolve_market(program_id, accounts, post_id, winning_emoji)
        }
        EmojimarketInstruction::ClaimPayout { post_id } => {
            msg!("ClaimPayout for post {}", post_id);
            process_claim_payout(program_id, accounts, post_id)
        }
    }
}
// ========== Helper to create and initialize an SPL Token account ==========
fn create_escrow_token_account<'a>(
	program_id: &Pubkey,
	payer_info: &AccountInfo<'a>,
	escrow_account_info: &AccountInfo<'a>,
	escrow_account_authority_seed: &[&[u8]],
	token_mint_info: &AccountInfo<'a>,
	system_program_info: &AccountInfo<'a>,
	token_program_info: &AccountInfo<'a>,
	rent_sysvar_info: &AccountInfo<'a>,
) -> ProgramResult {
	let rent = &Rent::from_account_info(rent_sysvar_info)?;
	let space = SplAccount::LEN;
	let lamports = rent.minimum_balance(space);

	let escrow_key = escrow_account_info.key;

	// create account (payer funds it)
	let create_ix = system_instruction::create_account(
		payer_info.key,
		escrow_key,
		lamports,
		space as u64,
		&spl_token::id(),
	);

   	// escrow PDA signer seeds for signing (if needed later)
	invoke(
		&create_ix,
        	&[
			payer_info.clone(),
			escrow_account_info.clone(),
			system_program_info.clone(),
        	],
    	)?;

    // initialize spl token account, owner = program PDA (we'll set owner later if needed)
    	let init_ix = token_instruction::initialize_account(
		token_program_info.key,
		escrow_account_info.key,
		token_mint_info.key,
		// owner of the token account should be the program-derived authority (PDA)
		// caller passes the PDA pubkey as the owner when creating the account data
		// here we assume the escrow_account_info has been created and owned by PDA
		// We set the owner to the program-derived authority PDA
		// For this helper, we will require the escrow account's owner argument to be already the PDA pubkey (passed in account's data)
		// To keep initialization deterministic, we use payer_info as rent payer and set owner param to the program PDA derived off seeds:
		// caller must have computed the PDA and made it the escrow_account_info key.
		// For simplicity here, we use payer as placeholder BUT caller MUST pass the actual PDA pubkey as the owner param in real use.
		// So we require a separate escrow_owner_pubkey param in production. Keep this note.
		payer_info.key,
    	)?;

    	invoke(
        	&init_ix,
		&[
			escrow_account_info.clone(),
			token_mint_info.clone(),
			payer_info.clone(), // rent sysvar
			token_program_info.clone(),
		],
    	)?;

    	Ok(())
}

fn process_create_post(
	_program_id: &Pubkey,
	accounts: &[AccountInfo],
	post_id: u64,
	start_ts: i64,
	end_ts: i64,
	creator_fee_bps: u16,
	cid: String,
	) -> ProgramResult {
	let account_info_iter = &mut accounts.iter();
	let payer = next_account_info(account_info_iter)?; // signer
	let post_account = next_account_info(account_info_iter)?; // PDA to hold Post
	// escrow_token_account, system_program, token_program, rent TODO

    	if !payer.is_signer {
        	return Err(ProgramError::MissingRequiredSignature);
   	}

    	if start_ts >= end_ts {
        	msg!("start_ts >= end_ts");
        	return Err(ProgramError::InvalidArgument);
   	}

    // create and initialize Post struct in post_account.data
	let mut post = Post {
		is_initialized: true,
		post_id,
		creator: *payer.key,
		start_ts,
		end_ts,
		pot_amount: 0,
		num_bets: 0,
		creator_fee_bps,
		creator_fees_withdrawable: 0,
		status: 0,
		cid_len: 0,
		cid: [0u8; crate::state::MAX_CID_BYTES],
	};

	let c = cid.as_bytes();
	if c.len() > crate::state::MAX_CID_BYTES {
		return Err(ProgramError::InvalidArgument);
	}
	post.cid_len = c.len() as u8;
	post.cid[..c.len()].copy_from_slice(c);

	// write to account
	let mut data = post_account.try_borrow_mut_data()?;
	post.serialize(&mut *data).map_err(|_| ProgramError::AccountDataTooSmall)?;

	Ok(())
}

fn process_place_bet(
	_program_id: &Pubkey,
	accounts: &[AccountInfo],
	post_id: u64,
	bet_id: u64,
	emoji_code: u32,
	quantity: u64,
	amount: u64,
	) -> ProgramResult {
	let account_info_iter = &mut accounts.iter();
	let bettor = next_account_info(account_info_iter)?; // signer
	let _bettor_token_account = next_account_info(account_info_iter)?;
	let post_account = next_account_info(account_info_iter)?; // mut
	let bet_account = next_account_info(account_info_iter)?; // mut
	// token_program etc omitted

	if !bettor.is_signer {
		return Err(ProgramError::MissingRequiredSignature);
	}

	// deserialize post
	let mut post_data = post_account.try_borrow_mut_data()?;
	let mut post = Post::try_from_slice(&post_data).map_err(|_| ProgramError::InvalidAccountData)?;

	// check that post is active
	let clock = Clock::get()?;
	let now_ts = clock.unix_timestamp;
    	if now_ts < post.start_ts || now_ts >= post.end_ts {
        	msg!("bet outside allowed window");
        	return Err(ProgramError::InvalidArgument);
    	}

	// TODO: verify amount matches on-chain pricing using supplies (not implemented in skeleton)

	// transfer USDC from bettor -> escrow via CPI to token program (omitted in skeleton)

	// create bet struct and store
	let bet = Bet {
		is_initialized: true,
		bet_id,
		post_id,
		user: *bettor.key,
		emoji_code,
		quantity,
		amount,
		timestamp: now_ts,
	};

	let mut bet_data = bet_account.try_borrow_mut_data()?;
	bet.serialize(&mut *bet_data).map_err(|_| ProgramError::AccountDataTooSmall)?;

	// update post summaries
	post.pot_amount = post.pot_amount.checked_add(amount).ok_or(ProgramError::InvalidAccountData)?;
	post.num_bets = post.num_bets.checked_add(1).ok_or(ProgramError::InvalidAccountData)?;

	// write updated post
	post.serialize(&mut *post_data).map_err(|_| ProgramError::AccountDataTooSmall)?;

	Ok(())
}

fn process_resolve_market(
	_program_id: &Pubkey,
	_accounts: &[AccountInfo],
	_post_id: u64,
	_winning_emoji: u32,
) -> ProgramResult {
	// For scalability, prefer pull-based claims: compute each winner's settlement and store a Claim record
	// Skeleton: omitted heavy-lifting. Implementation should either batch-push payouts via CPI or set settlements
	Err(ProgramError::Custom(0))
}

fn process_claim_payout(
	_program_id: &Pubkey,
	_accounts: &[AccountInfo],
	_post_id: u64,
) -> ProgramResult {
	// User claims their payout. CPI to transfer USDC from escrow -> user token account.
	Err(ProgramError::Custom(1))
}
