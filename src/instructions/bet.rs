use crate::error::ErrorCode;
use crate::math::{
    calculate_malus, calculate_quadratic_uplift, calculate_total_cost, calculate_unit_price,
};
use crate::state::{BetAccount, Market};
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
#[instruction(market_id: u64)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        seeds = [b"market", market.creator.as_ref(), &market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init_if_needed,
        payer = user,
        space = BetAccount::space(),
        seeds = [b"bet", market.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, BetAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PlaceBet>, emoji_id: u32, vote_qty: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let bet = &mut ctx.accounts.bet;
    let user = &ctx.accounts.user;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    // Validate vote_qty >= 1
    require!(vote_qty >= 1, ErrorCode::InvalidVoteQuantity);

    // Require market Active and now < end_ts
    require!(market.status == 0, ErrorCode::MarketNotActive);
    require!(now < market.end_ts, ErrorCode::MarketEnded);

    // Calculate time progress
    let elapsed = now
        .checked_sub(market.start_ts)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;
    let total_duration = market
        .end_ts
        .checked_sub(market.start_ts)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;

    // Calculate malus
    let malus_nanos = calculate_malus(elapsed, total_duration, market.malus_k_millis)?;

    // Calculate quadratic uplift based on current total votes
    let quad_multiplier_nanos = calculate_quadratic_uplift(
        market.total_votes,
        market.quad_a_micros,
        market.quad_b_micros,
    )?;

    // Calculate unit price
    let unit_price = calculate_unit_price(
        market.base_price_lamports,
        malus_nanos,
        quad_multiplier_nanos,
    )?;

    // Calculate total cost
    let total_cost = calculate_total_cost(unit_price, vote_qty)?;

    // Transfer SOL from user to market account
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: user.to_account_info(),
            to: market.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, total_cost)?;

    // Update market totals
    market.total_pot = market
        .total_pot
        .checked_add(total_cost)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    market.total_votes = market
        .total_votes
        .checked_add(vote_qty)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Update emoji votes in market
    if let Some(idx) = market.emoji_ids.iter().position(|&id| id == emoji_id) {
        // Emoji already exists, increment votes
        market.emoji_votes[idx] = market.emoji_votes[idx]
            .checked_add(vote_qty)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    } else {
        // New emoji, add to lists
        market.emoji_ids.push(emoji_id);
        market.emoji_votes.push(vote_qty);
    }

    // Initialize bet account if needed
    if bet.market == Pubkey::default() {
        bet.market = market.key();
        bet.user = user.key();
        bet.emoji_ids = Vec::new();
        bet.emoji_votes = Vec::new();
        bet.total_spent = 0;
        bet.claimed = false;
        bet.bump = ctx.bumps.bet;
    }

    // Update user's bet
    if let Some(idx) = bet.emoji_ids.iter().position(|&id| id == emoji_id) {
        // User already voted for this emoji, increment
        bet.emoji_votes[idx] = bet.emoji_votes[idx]
            .checked_add(vote_qty)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    } else {
        // First vote for this emoji
        bet.emoji_ids.push(emoji_id);
        bet.emoji_votes.push(vote_qty);
    }

    bet.total_spent = bet
        .total_spent
        .checked_add(total_cost)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    Ok(())
}
