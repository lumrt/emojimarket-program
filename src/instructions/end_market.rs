use crate::error::ErrorCode;
use crate::math::calculate_fee;
use crate::state::{Config, Market};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(market_id: u64)]
pub struct EndMarket<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"market", market.creator.as_ref(), &market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub caller: Signer<'info>,

    /// CHECK: Platform admin to receive fees
    #[account(mut)]
    pub platform_admin: AccountInfo<'info>,

    /// CHECK: Market creator to receive fees
    #[account(mut)]
    pub market_creator: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EndMarket>) -> Result<()> {
    let config = &ctx.accounts.config;
    let market = &mut ctx.accounts.market;
    let caller = &ctx.accounts.caller;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    // Validate caller is admin or creator
    let is_admin = caller.key() == config.admin_address;
    let is_creator = caller.key() == market.creator;
    require!(is_admin || is_creator, ErrorCode::Unauthorized);

    // Require now >= end_ts
    require!(now >= market.end_ts, ErrorCode::MarketNotEnded);

    // Require status = Active
    require!(market.status == 0, ErrorCode::MarketNotActive);

    // Find winner: emoji with highest votes (tie-break: lowest emoji_id)
    let mut winner_emoji: Option<u32> = None;
    let mut max_votes: u64 = 0;

    for (idx, &votes) in market.emoji_votes.iter().enumerate() {
        let emoji_id = market.emoji_ids[idx];
        if votes > max_votes
            || (votes == max_votes && (winner_emoji.is_none() || emoji_id < winner_emoji.unwrap()))
        {
            max_votes = votes;
            winner_emoji = Some(emoji_id);
        }
    }

    market.winner = winner_emoji;

    // Compute fees
    let platform_fee = calculate_fee(market.total_pot, market.platform_fee_bps)?;
    let creator_fee = calculate_fee(market.total_pot, market.creator_fee_bps)?;

    market.platform_fee_taken = platform_fee;
    market.creator_fee_taken = creator_fee;

    // Transfer platform fee
    if platform_fee > 0 {
        **market.to_account_info().try_borrow_mut_lamports()? = market
            .to_account_info()
            .lamports()
            .checked_sub(platform_fee)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;

        **ctx.accounts.platform_admin.try_borrow_mut_lamports()? = ctx
            .accounts
            .platform_admin
            .lamports()
            .checked_add(platform_fee)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }

    // Transfer creator fee
    if creator_fee > 0 {
        **market.to_account_info().try_borrow_mut_lamports()? = market
            .to_account_info()
            .lamports()
            .checked_sub(creator_fee)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;

        **ctx.accounts.market_creator.try_borrow_mut_lamports()? = ctx
            .accounts
            .market_creator
            .lamports()
            .checked_add(creator_fee)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }

    // Mark status = Ended
    market.status = 1;

    Ok(())
}
