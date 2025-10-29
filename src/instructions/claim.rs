use crate::error::ErrorCode;
use crate::math::calculate_user_share;
use crate::state::{BetAccount, Market};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(market_id: u64)]
pub struct Claim<'info> {
    #[account(
        mut,
        seeds = [b"market", market.creator.as_ref(), &market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [b"bet", market.key().as_ref(), user.key().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, BetAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Claim>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let bet = &mut ctx.accounts.bet;
    let user = &ctx.accounts.user;

    // Require market Ended
    require!(market.status == 1, ErrorCode::MarketNotEnded);

    // Require not yet claimed
    require!(!bet.claimed, ErrorCode::AlreadyClaimed);

    // Get winner emoji
    let winner_emoji = market.winner.ok_or(ErrorCode::NoWinningVotes)?;

    // Find user's votes on winning emoji
    let user_winning_votes = bet
        .emoji_ids
        .iter()
        .position(|&id| id == winner_emoji)
        .and_then(|idx| Some(bet.emoji_votes[idx]))
        .unwrap_or(0);

    // Require user has votes on winning emoji
    require!(user_winning_votes > 0, ErrorCode::NoWinningVotes);

    // Find total winning votes
    let total_winning_votes = market
        .emoji_ids
        .iter()
        .position(|&id| id == winner_emoji)
        .and_then(|idx| Some(market.emoji_votes[idx]))
        .unwrap_or(0);

    require!(total_winning_votes > 0, ErrorCode::InvalidCalculation);

    // Calculate payout pool = pot - fees
    let payout_pool = market
        .total_pot
        .checked_sub(market.platform_fee_taken)
        .ok_or(ErrorCode::ArithmeticUnderflow)?
        .checked_sub(market.creator_fee_taken)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;

    // Calculate user share
    let user_share = calculate_user_share(payout_pool, user_winning_votes, total_winning_votes)?;

    // Transfer from market to user
    if user_share > 0 {
        **market.to_account_info().try_borrow_mut_lamports()? = market
            .to_account_info()
            .lamports()
            .checked_sub(user_share)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;

        **user.to_account_info().try_borrow_mut_lamports()? = user
            .to_account_info()
            .lamports()
            .checked_add(user_share)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }

    // Mark claimed
    bet.claimed = true;

    Ok(())
}
