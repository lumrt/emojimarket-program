use crate::instruction::EmojimarketInstruction;
use crate::state::{Post, Bet, PostStatus};
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar, rent::Rent},
    program::{invoke_signed, invoke},
    system_instruction,
};
use spl_token::instruction as token_instruction;
use spl_token::state::Account as SplAccount;

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
        EmojimarketInstruction::ResolveMarket { post_id, winner_emoji } => {
            msg!("ResolveMarket {} -> {}", post_id, winner_emoji);
            process_resolve_market(program_id, accounts, post_id, winner_emoji)
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
	payer: &AccountInfo<'a>,
	escrow_account: &AccountInfo<'a>,
	token_mint: &AccountInfo<'a>,
	system_program: &AccountInfo<'a>,
	token_program: &AccountInfo<'a>,
	rent_sysvar: &AccountInfo<'a>,
	post_id: u64,
) -> ProgramResult {
	let rent = Rent::from_account_info(rent_sysvar)?;
	let space = SplAccount::LEN;
	let lamports = rent.minimum_balance(space);

	// derive PDA for escrow
	let (escrow_pda, bump) =
		Pubkey::find_program_address(&[b"escrow", &post_id.to_le_bytes()], program_id);

	if escrow_pda != *escrow_account.key {
		msg!("Escrow PDA mismatch");
		return Err(ProgramError::InvalidSeeds);
	}

	// create escrow account owned by SPL Token program
	let create_ix = system_instruction::create_account(
		payer.key,
		escrow_account.key,
		lamports,
		space as u64,
		&spl_token::id(),
	);

	invoke(
		&create_ix,
		&[
		payer.clone(),
		escrow_account.clone(),
		system_program.clone(),
		],
	)?;

	// initialize escrow SPL token account, owner = escrow_pda
	let init_ix = token_instruction::initialize_account3(
		token_program.key,
		escrow_account.key,
		token_mint.key,
		&escrow_pda,
	)?;

	invoke_signed(
		&init_ix,
		&[
		escrow_account.clone(),
		token_mint.clone(),
		token_program.clone(),
		rent_sysvar.clone(),
		],
		&[&[b"escrow", &post_id.to_le_bytes(), &[bump]]],
	)?;

	Ok(())
}

fn process_create_post(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	post_id: u64,
	start_ts: i64,
	end_ts: i64,
	creator_fee_bps: u16,
	cid: String,
	) -> ProgramResult {
	let account_info_iter = &mut accounts.iter();
	let creator = next_account_info(account_info_iter)?;
	let post_account = next_account_info(account_info_iter)?;
	let escrow_account = next_account_info(account_info_iter)?;
	let token_mint = next_account_info(account_info_iter)?;
	let system_program = next_account_info(account_info_iter)?;
	let token_program = next_account_info(account_info_iter)?;
	let rent_sysvar = next_account_info(account_info_iter)?;

	if !creator.is_signer {
		return Err(ProgramError::MissingRequiredSignature);
	}

	// derive post PDA and check it matches
	let (post_pda, _) = Pubkey::find_program_address(&[b"post", &post_id.to_le_bytes()], program_id);
	if post_pda != *post_account.key {
		msg!("❌ Post PDA mismatch");
		return Err(ProgramError::InvalidSeeds);
	}

	// derive escrow PDA and validate
	let (escrow_pda, _) =
		Pubkey::find_program_address(&[b"escrow", &post_id.to_le_bytes()], program_id);
	if escrow_pda != *escrow_account.key {
		msg!("❌ Escrow PDA mismatch");
		return Err(ProgramError::InvalidSeeds);
	}

	// initialize post account data
	let mut post_data = Post::try_from_slice(&post_account.data.borrow())?;
	post_data.is_initialized = true;
	post_data.post_id = post_id;
	post_data.start_ts = start_ts;
	post_data.end_ts = end_ts;
	post_data.creator = *creator.key;
	post_data.creator_fee_bps = creator_fee_bps;
	post_data.creator_fees_withdrawable = 0;
	
	// Convert CID string to bytes array
	let cid_bytes = cid.as_bytes();
	let cid_len = cid_bytes.len().min(crate::state::MAX_CID_BYTES);
	post_data.cid_len = cid_len as u8;
	post_data.cid[..cid_len].copy_from_slice(&cid_bytes[..cid_len]);
	
	post_data.status = PostStatus::Open;
	post_data.pot_amount = 0;
	post_data.num_bets = 0;
	post_data.escrow_account = *escrow_account.key;

	post_data.serialize(&mut &mut post_account.data.borrow_mut()[..])?;

	// create escrow SPL token account PDA
	create_escrow_token_account(
		program_id,
		creator,
		escrow_account,
		token_mint,
		system_program,
		token_program,
		rent_sysvar,
		post_id,
	)?;

	msg!("✅ Post {} created successfully with escrow PDA {}", post_id, escrow_pda);
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
